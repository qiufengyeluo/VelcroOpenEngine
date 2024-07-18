
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    ops::{Index, IndexMut},
    sync::atomic::{self, AtomicIsize},
};
use crate::reflect::prelude::{Reflect, ReflectArray, ReflectList, ReflectHashMap, FieldInfo};

mod memory_block;
mod handle;

pub use handle::*;
pub use memory_block::*;

const INVALID_GENERATION: u32 = 0;

/// Pool allows to create as many objects as you want in contiguous memory
/// block. It allows to create and delete objects much faster than if they'll
/// be allocated on heap. Also since objects stored in contiguous memory block
/// they can be effectively accessed because such memory layout is cache-friendly.
#[derive(Debug)]
pub struct Allocator<T, M = Option<T>>
where
    T: Sized,
    M: MemoryBlockContainer<Element = T>,
{
    records: Vec<AllocatorRecord<T, M>>,
    free_stack: Vec<u32>,
}


impl<T, M> Reflect for Allocator<T, M>
where
    T: Reflect,
    M: MemoryBlockContainer<Element = T> + Reflect,
{
    #[inline]
    fn source_path() -> &'static str {
        file!()
    }

    #[inline]
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    #[inline]
    fn doc(&self) -> &'static str {
        ""
    }

    #[inline]
    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        func(&[])
    }

    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    #[inline]
    fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
        func(self)
    }

    #[inline]
    fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
        func(self)
    }

    #[inline]
    fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
        func(self)
    }

    #[inline]
    fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
        func(self)
    }

    #[inline]
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        let this = std::mem::replace(self, value.take()?);
        Ok(Box::new(this))
    }

    fn assembly_name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn type_assembly_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    #[inline]
    fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
        func(Some(self))
    }

    #[inline]
    fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
        func(Some(self))
    }
}


impl<T, M> ReflectArray for Allocator<T, M>
where
    T: Reflect,
    M: MemoryBlockContainer<Element = T> + Reflect,
{
    #[inline]
    fn reflect_index(&self, index: usize) -> Option<&dyn Reflect> {
        self.at(index as u32).map(|p| p as &dyn Reflect)
    }

    #[inline]
    fn reflect_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        self.at_mut(index as u32).map(|p| p as &mut dyn Reflect)
    }

    #[inline]
    fn reflect_len(&self) -> usize {
        self.get_capacity() as usize
    }
}

impl<T, P> PartialEq for Allocator<T, P>
where
    T: PartialEq,
    P: MemoryBlockContainer<Element = T> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.records == other.records
    }
}

// Zero - non-borrowed.
// Negative values - amount of mutable borrows, positive - amount of immutable borrows.
#[derive(Default, Debug)]
struct RefCounter(pub AtomicIsize);

impl RefCounter {
    fn increment(&self) {
        self.0.fetch_add(1, atomic::Ordering::Relaxed);
    }

    fn decrement(&self) {
        self.0.fetch_sub(1, atomic::Ordering::Relaxed);
    }
}

#[derive(Debug)]
struct AllocatorRecord<T, M = Option<T>>
where
    T: Sized,
    M: MemoryBlockContainer<Element = T>,
{
    refc: RefCounter,
    // Generation number, used to keep info about lifetime. The handle is valid
    // only if record it points to is of the same generation as the pool record.
    // Notes: Zero is unknown generation used for None handles.
    generation: u32,
    // Actual memory block.
    block: MemoryBlock<M>,
}

impl<T, M> PartialEq for AllocatorRecord<T, M>
where
    T: PartialEq,
    M: MemoryBlockContainer<Element = T> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && self.block.get() == other.block.get()
    }
}

impl<T, P> Default for AllocatorRecord<T, P>
where
    P: MemoryBlockContainer<Element = T> + 'static,
{
    #[inline]
    fn default() -> Self {
        Self {
            refc: Default::default(),
            generation: INVALID_GENERATION,
            block: MemoryBlock::new_empty(),
        }
    }
}



impl<T, M> Default for Allocator<T, M>
where
    T: 'static,
    M: MemoryBlockContainer<Element = T> + 'static,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Ticket<T> {
    index:  u32,
    marker: PhantomData<T>,
}

impl<T> Drop for Ticket<T> {
    fn drop(&mut self) {
        panic!(
            "An object at index {} must be returned to a pool it was taken from! \
            Call Allocator::forget_ticket if you don't need the object anymore.",
            self.index
        )
    }
}

impl<T: Clone> Clone for AllocatorRecord<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            refc: Default::default(),
            generation: self.generation,
            block: self.block.clone(),
        }
    }
}

impl<T: Clone> Clone for Allocator<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            records: self.records.clone(),
            free_stack: self.free_stack.clone(),
        }
    }
}

impl<T, M> Allocator<T, M>
where
    M: MemoryBlockContainer<Element = T> + 'static,
{
    #[inline]
    pub fn new() -> Self {
        Allocator {
            records: Vec::new(),
            free_stack: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: u32) -> Self {
        let capacity = usize::try_from(capacity).expect("capacity overflowed usize");
        Allocator {
            records: Vec::with_capacity(capacity),
            free_stack: Vec::new(),
        }
    }

    fn records_len(&self) -> u32 {
        u32::try_from(self.records.len()).expect("Number of records overflowed u32")
    }

    fn records_get(&self, index: u32) -> Option<&AllocatorRecord<T, M>> {
        let index = usize::try_from(index).expect("Index overflowed usize");
        self.records.get(index)
    }

    fn records_get_mut(&mut self, index: u32) -> Option<&mut AllocatorRecord<T, M>> {
        let index = usize::try_from(index).expect("Index overflowed usize");
        self.records.get_mut(index)
    }


     /// Forgets that value at ticket was reserved and makes it usable again.
    /// Useful when you don't need to put value back by ticket, but just make
    /// allocator record usable again.
    #[inline]
    pub fn forget_ticket(&mut self, ticket: Ticket<T>) {
        self.free_stack.push(ticket.index);
        std::mem::forget(ticket);
    }

    /// Returns total capacity of pool. Capacity has nothing about real amount of objects in pool!
    #[inline]
    #[must_use]
    pub fn get_capacity(&self) -> u32 {
        u32::try_from(self.records.len()).expect("records.len() overflowed u32")
    }

     /// Destroys all objects in pool. All handles to objects will become invalid.
    ///
    /// # Remarks
    ///
    /// Use this method cautiously if objects in pool have cross "references" (handles)
    /// to each other. This method will make all produced handles invalid and any further
    /// calls for [`borrow`](Self::borrow) or [`borrow_mut`](Self::borrow_mut) will raise panic.
    #[inline]
    pub fn clear(&mut self) {
        self.records.clear();
        self.free_stack.clear();
    }

    #[inline]
    #[must_use]
    pub fn at_mut(&mut self, n: u32) -> Option<&mut T> {
        self.records_get_mut(n).and_then(|rec| rec.block.as_mut())
    }

    #[inline]
    #[must_use]
    pub fn at(&self, n: u32) -> Option<&T> {
        self.records_get(n)
            .and_then(|rec| rec.block.get().as_ref())
    }

    /// Returns the exact number of "alive" objects in the pool.
    ///
    /// Records that have been reserved (e.g. by [`take_reserve`]) are *not* counted.
    ///
    /// It iterates through the entire pool to count the live objects so the complexity is `O(n)`.
    ///
    /// See also [`total_count`].
    ///
    /// # Example
    ///
    /// ```
    /// use velcro_rtti::memory::Allocator;
    /// let mut allocator = Allocator::<u32>::new();
    /// allocator.spawn(123);
    /// allocator.spawn(321);
    /// assert_eq!(allocator.alive_count(), 2);
    /// ```
    ///
    /// [`take_reserve`]: Allocator::take_reserve
    /// [`total_count`]: Allocator::total_count
    #[inline]
    #[must_use]
    pub fn alive_count(&self) -> u32 {
        let cnt = self.iter().count();
        u32::try_from(cnt).expect("alive_count overflowed u32")
    }

    /// Returns the number of allocated objects in the Allocator.
    ///
    /// It also counts records that have been reserved (e.g. by [`take_reserve`]).
    ///
    /// This method is `O(1)`.
    ///
    /// See also [`alive_count`].
    ///
    /// [`take_reserve`]: Allocator::take_reserve
    /// [`alive_count`]: Allocator::alive_count
    #[inline]
    pub fn total_count(&self) -> u32 {
        let free = u32::try_from(self.free_stack.len()).expect("free stack length overflowed u32");
        self.records_len() - free
    }


     /// Returns a reference to the first element in the pool (if any).
     pub fn first_ref(&self) -> Option<&T> {
        self.iter().next()
    }

    /// Returns a reference to the first element in the pool (if any).
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.iter_mut().next()
    }


    /// Creates new memory iterator that iterates over filled records in Allocator.
    ///
    /// # Example
    ///
    /// ```
    /// use velcro_rtti::memory::Allocator;
    /// let mut allocator = Allocator::<u32>::new();
    /// allocator.spawn(123);
    /// allocator.spawn(321);
    /// let mut iter = allocator.iter();
    /// assert_eq!(*iter.next().unwrap(), 123);
    /// assert_eq!(*iter.next().unwrap(), 321);
    /// ```
    #[must_use]
    #[inline]
    pub fn iter(&self) -> AllocatorIterator<T, M> {
        unsafe {
            AllocatorIterator {
                ptr: self.records.as_ptr(),
                end: self.records.as_ptr().add(self.records.len()),
                marker: PhantomData,
            }
        }
    }

    /// Creates new pair iterator that iterates over filled records using pair (handle, payload)
    /// Can be useful when there is a need to iterate over pool records and know a handle of
    /// that record.
    #[inline]
    pub fn pair_iter(&self) -> AllocatorPairIterator<T, M> {
        AllocatorPairIterator {
            allocator: self,
            current: 0,
        }
    }

    /// Creates new pool iterator that iterates over filled records in pool allowing
    /// to modify record payload.
    ///
    /// # Example
    ///
    /// ```
    /// use velcro_rtti::memory::Allocator;
    /// let mut allocator = Allocator::<u32>::new();
    /// allocator.spawn(123);
    /// allocator.spawn(321);
    /// let mut iter = allocator.iter_mut();
    /// assert_eq!(*iter.next().unwrap(), 123);
    /// assert_eq!(*iter.next().unwrap(), 321);
    /// ```
    #[must_use]
    #[inline]
    pub fn iter_mut(&mut self) -> AllocatorIteratorMut<T, M> {
        unsafe {
            AllocatorIteratorMut {
                ptr: self.records.as_mut_ptr(),
                end: self.records.as_mut_ptr().add(self.records.len()),
                marker: PhantomData,
            }
        }
    }

    /// Creates new pair iterator that iterates over filled records using pair (handle, payload)
    /// Can be useful when there is a need to iterate over pool records and know a handle of
    /// that record.
    #[inline]
    pub fn pair_iter_mut(&mut self) -> AllocatorPairIteratorMut<T, M> {
        unsafe {
            AllocatorPairIteratorMut {
                current: 0,
                ptr: self.records.as_mut_ptr(),
                end: self.records.as_mut_ptr().add(self.records.len()),
                marker: PhantomData,
            }
        }
    }

    /// Retains pool records selected by `pred`. Useful when you need to remove all pool records
    /// by some criteria.
    #[inline]
    pub fn retain<F>(&mut self, mut pred: F)
    where
        F: FnMut(&T) -> bool,
    {
        for (i, record) in self.records.iter_mut().enumerate() {
            if record.generation == INVALID_GENERATION {
                continue;
            }

            let retain = if let Some(block) = record.block.as_ref() {
                pred(block)
            } else {
                continue;
            };

            if !retain {
                self.free_stack.push(i as u32);
                record.block.take(); // and Drop
            }
        }
    }
}


pub struct AllocatorIterator<'a, T, M>
where
    M: MemoryBlockContainer<Element = T>,
{
    ptr: *const AllocatorRecord<T, M>,
    end: *const AllocatorRecord<T, M>,
    marker: PhantomData<&'a T>,
}

impl<'a, T, M> Iterator for AllocatorIterator<'a, T, M>
where
    M: MemoryBlockContainer<Element = T> + 'static,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.ptr != self.end {
                let current = &*self.ptr;
                if let Some(block) = current.block.as_ref() {
                    self.ptr = self.ptr.offset(1);
                    return Some(block);
                }
                self.ptr = self.ptr.offset(1);
            }

            None
        }
    }
}

pub struct AllocatorPairIterator<'a, T, M: MemoryBlockContainer<Element = T>> {
    allocator: &'a Allocator<T, M>,
    current: usize,
}

impl<'a, T, M> Iterator for AllocatorPairIterator<'a, T, M>
where
    M: MemoryBlockContainer<Element = T>,
{
    type Item = (Handle<T>, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.allocator.records.get(self.current) {
                Some(record) => {
                    if let Some(block) = record.block.as_ref() {
                        let handle = Handle::new(self.current as u32, record.generation);
                        self.current += 1;
                        return Some((handle, block));
                    }
                    self.current += 1;
                }
                None => return None,
            }
        }
    }
}

pub struct AllocatorIteratorMut<'a, T, M>
where
M: MemoryBlockContainer<Element = T>,
{
    ptr: *mut AllocatorRecord<T, M>,
    end: *mut AllocatorRecord<T, M>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T, M> Iterator for AllocatorIteratorMut<'a, T, M>
where
M: MemoryBlockContainer<Element = T> + 'static,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.ptr != self.end {
                let current = &mut *self.ptr;
                if let Some(block) = current.block.as_mut() {
                    self.ptr = self.ptr.offset(1);
                    return Some(block);
                }
                self.ptr = self.ptr.offset(1);
            }

            None
        }
    }
}

pub struct AllocatorPairIteratorMut<'a, T, M>
where
    M: MemoryBlockContainer<Element = T>,
{
    ptr: *mut AllocatorRecord<T, M>,
    end: *mut AllocatorRecord<T, M>,
    marker: PhantomData<&'a mut T>,
    current: usize,
}

impl<'a, T, M> Iterator for AllocatorPairIteratorMut<'a, T, M>
where
    M: MemoryBlockContainer<Element = T> + 'static,
{
    type Item = (Handle<T>, &'a mut T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.ptr != self.end {
                let current = &mut *self.ptr;
                if let Some(block) = current.block.as_mut() {
                    let handle = Handle::new(self.current as u32, current.generation);
                    self.ptr = self.ptr.offset(1);
                    self.current += 1;
                    return Some((handle, block));
                }
                self.ptr = self.ptr.offset(1);
                self.current += 1;
            }

            None
        }
    }
}