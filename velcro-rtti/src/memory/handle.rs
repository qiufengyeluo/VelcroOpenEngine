use crate::memory::*;
use crate::type_traits::{combine_uuids, TypeUuidProvider};
use crate::{reflect::prelude::*, uuid_provider};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::atomic::{self, AtomicUsize},
};
use velcro_utils::UUID;

/// Handle is some sort of non-owning reference to content in a pool. It stores
/// index of object and additional information that allows to ensure that handle
/// is still valid (points to the same object as when handle was created).
#[derive(Reflect, Serialize, Deserialize)]
pub struct Handle<T> {
    /// Index of object in pool.
    #[reflect(read_only, description = "Index of an object in a pool.")]
    pub(super) index: u32,
    /// Generation number, if it is same as generation of pool record at
    /// index of handle then this is valid handle.
    #[reflect(read_only, description = "Generation of an object in a pool.")]
    pub(super) generation: u32,
    /// Type holder.
    #[reflect(hidden)]
    pub(super) type_marker: PhantomData<T>,
}

impl<T> Copy for Handle<T> {}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    #[inline]
    fn eq(&self, other: &Handle<T>) -> bool {
        self.generation == other.generation && self.index == other.index
    }
}

impl<T> Hash for Handle<T> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.index.hash(state);
        self.generation.hash(state);
    }
}

impl<T> Handle<T> {
    pub const NONE: Handle<T> = Handle {
        index: 0,
        generation: INVALID_GENERATION,
        type_marker: PhantomData,
    };

    #[inline(always)]
    pub fn is_none(self) -> bool {
        self.index == 0 && self.generation == INVALID_GENERATION
    }

    #[inline(always)]
    pub fn is_some(self) -> bool {
        !self.is_none()
    }

    #[inline(always)]
    pub fn index(self) -> u32 {
        self.index
    }

    #[inline(always)]
    pub fn generation(self) -> u32 {
        self.generation
    }

    #[inline(always)]
    pub fn new(index: u32, generation: u32) -> Self {
        Handle {
            index,
            generation,
            type_marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn transmute<U>(&self) -> Handle<U> {
        Handle {
            index: self.index,
            generation: self.generation,
            type_marker: Default::default(),
        }
    }

    #[inline(always)]
    pub fn decode_from_u128(num: u128) -> Self {
        Self {
            index: num as u32,
            generation: (num >> 32) as u32,
            type_marker: Default::default(),
        }
    }

    #[inline(always)]
    pub fn encode_to_u128(&self) -> u128 {
        (self.index as u128) | ((self.generation as u128) << 32)
    }
}

impl<T> TypeUuidProvider for Handle<T>
where
    T: TypeUuidProvider,
{
    #[inline]
    fn type_uuid() -> UUID {
        combine_uuids(
            UUID::create_string("{9a1ca42b-6bf1-ee7d-417f-f8b00e71928b}"),
            T::type_uuid(),
        )
    }
}

impl<T> PartialOrd for Handle<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Handle<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

impl<T> Display for Handle<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.index, self.generation)
    }
}

/// Atomic handle.
pub struct AtomicHandle(AtomicUsize);

impl Clone for AtomicHandle {
    #[inline]
    fn clone(&self) -> Self {
        Self(AtomicUsize::new(self.0.load(atomic::Ordering::Relaxed)))
    }
}

impl Default for AtomicHandle {
    #[inline]
    fn default() -> Self {
        Self::none()
    }
}

impl AtomicHandle {
    #[inline]
    pub fn none() -> Self {
        Self(AtomicUsize::new(0))
    }

    #[inline]
    pub fn new(index: u32, generation: u32) -> Self {
        let handle = Self(AtomicUsize::new(0));
        handle.set(index, generation);
        handle
    }

    #[inline]
    pub fn set(&self, index: u32, generation: u32) {
        let index = (index as usize) << (usize::BITS / 2) >> (usize::BITS / 2);
        let generation = (generation as usize) << (usize::BITS / 2);
        self.0.store(index | generation, atomic::Ordering::Relaxed);
    }

    #[inline]
    pub fn set_from_handle<T>(&self, handle: Handle<T>) {
        self.set(handle.index, handle.generation)
    }

    #[inline(always)]
    pub fn is_some(&self) -> bool {
        self.generation() != INVALID_GENERATION
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[inline]
    pub fn index(&self) -> u32 {
        let bytes = self.0.load(atomic::Ordering::Relaxed);
        ((bytes << (usize::BITS / 2)) >> (usize::BITS / 2)) as u32
    }

    #[inline]
    pub fn generation(&self) -> u32 {
        let bytes = self.0.load(atomic::Ordering::Relaxed);
        (bytes >> (usize::BITS / 2)) as u32
    }
}

impl Display for AtomicHandle {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.index(), self.generation())
    }
}

impl Debug for AtomicHandle {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Idx: {}; Gen: {}]", self.index(), self.generation())
    }
}

/// Type-erased handle.
#[derive(
    Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize,
)]
pub struct ErasedHandle {
    /// Index of object in pool.
    #[reflect(read_only)]
    index: u32,
    /// Generation number, if it is same as generation of pool record at
    /// index of handle then this is valid handle.
    #[reflect(read_only)]
    generation: u32,
}

uuid_provider!(ErasedHandle = "50131acc-8b3b-40b5-b495-e2c552c94db3");

impl Display for ErasedHandle {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.index, self.generation)
    }
}

impl Default for ErasedHandle {
    #[inline]
    fn default() -> Self {
        Self::none()
    }
}

impl<T> From<ErasedHandle> for Handle<T> {
    #[inline]
    fn from(erased_handle: ErasedHandle) -> Self {
        Handle {
            index: erased_handle.index,
            generation: erased_handle.generation,
            type_marker: PhantomData,
        }
    }
}

impl<T> From<AtomicHandle> for Handle<T> {
    #[inline]
    fn from(atomic_handle: AtomicHandle) -> Self {
        Handle {
            index: atomic_handle.index(),
            generation: atomic_handle.generation(),
            type_marker: PhantomData,
        }
    }
}

impl<T> From<Handle<T>> for ErasedHandle {
    #[inline]
    fn from(h: Handle<T>) -> Self {
        Self {
            index: h.index,
            generation: h.generation,
        }
    }
}

impl ErasedHandle {
    #[inline]
    pub fn none() -> Self {
        Self {
            index: 0,
            generation: INVALID_GENERATION,
        }
    }

    #[inline]
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    #[inline(always)]
    pub fn is_some(&self) -> bool {
        self.generation != INVALID_GENERATION
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[inline(always)]
    pub fn index(self) -> u32 {
        self.index
    }

    #[inline(always)]
    pub fn generation(self) -> u32 {
        self.generation
    }
}

impl<T> Default for Handle<T> {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl<T> Debug for Handle<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Idx: {}; Gen: {}]", self.index, self.generation)
    }
}
