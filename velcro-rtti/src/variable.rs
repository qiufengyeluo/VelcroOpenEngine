use crate::{
    reflect::prelude::*,
    visitor::{prelude::*, VisitorFlags},
};

use bitflags::{bitflags, Flags};
use std::{
    any::{Any, TypeId},
    cell::Cell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use velcro_derive::Reflect;


#[derive(Reflect, Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[repr(transparent)]
pub struct VariableFlags(u8);

bitflags! {
    impl VariableFlags: u8 {
        /// Nothing.
        const NONE = 0;
        /// A variable was externally modified.
        const MODIFIED = 0b0000_0001;
        /// A variable must be synced with respective variable from data model.
        const NEED_SYNC = 0b0000_0010;
    }
}

impl Debug for VariableFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == VariableFlags::NONE {
            write!(f, "NONE")
        } else {
            for (i, flag) in self.iter().enumerate() {
                if i != 0 {
                    write!(f, "|")?
                }
                match flag {
                    VariableFlags::MODIFIED => write!(f, "MOD")?,
                    VariableFlags::NEED_SYNC => write!(f, "SYNC")?,
                    _ => {}
                }
            }
            Ok(())
        }
    }
}


/// An error that could occur during inheritance.
#[derive(Debug)]
pub enum InheritError {
    /// Types of properties mismatch.
    TypesMismatch {
        /// Type of left property.
        left_type: &'static str,
        /// Type of right property.
        right_type: &'static str,
    },
}

/// 变量的包装器，包含附加标志，表明初始值在运行时已更改.
///
/// InheritableVariables 用于资源继承系统.资源继承可能听起来很奇怪, 但其背后的想法非常简单 -
/// 如果 current 中的值在运行时没有更改, 则从父资源获取属性值.
///
/// To get better understanding, let's look at very simple example. Imagine you have a scene with a 3d model
/// instance. Now you realizes that the 3d model has a misplaced object and you need to fix it, you open a
/// 3D modelling software (Blender, 3Ds max, etc) and move the object to a correct spot and re-save the 3D model.
/// The question is: what should happen with the instance of the object in the scene? Logical answer would be:
/// if it hasn't been modified, then just take the new position from the 3D model. This is where inheritable
/// variable comes into play. If you've change the value of such variable, it will remember changes and the object
/// will stay on its new position instead of changed.
///
/// # Deref and DerefMut
///
/// Access via Deref provides access to inner variable. **DerefMut marks variable as modified** and returns a
/// mutable reference to inner variable.
pub struct InheritableVariable<T> {
    value: T,
    flags: Cell<VariableFlags>,
}

impl<T: Debug> Debug for InheritableVariable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (flags:{:?})", self.value, self.flags.get())
    }
}

impl<T: Clone> Clone for InheritableVariable<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            flags: self.flags.clone(),
        }
    }
}

impl<T> From<T> for InheritableVariable<T> {
    #[inline]
    fn from(v: T) -> Self {
        InheritableVariable::new_modified(v)
    }
}

impl<T: PartialEq> PartialEq for InheritableVariable<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // `custom` flag intentionally ignored!
        self.value.eq(&other.value)
    }
}

impl<T: Eq> Eq for InheritableVariable<T> {}

impl<T: Default> Default for InheritableVariable<T> {
    #[inline]
    fn default() -> Self {
        Self {
            value: T::default(),
            flags: Cell::new(VariableFlags::MODIFIED),
        }
    }
}

impl<T: Clone> InheritableVariable<T> {
    /// Clones wrapped value.
    #[inline]
    pub fn clone_inner(&self) -> T {
        self.value.clone()
    }

    /// Tries to sync a value in a data model with a value in the inheritable variable. The value
    /// will be synced only if it was marked as needs sync.
    #[inline]
    pub fn try_sync_model<S: FnOnce(T)>(&self, setter: S) -> bool {
        if self.need_sync() {
            // Drop flag first.
            let mut flags = self.flags.get();
            flags.remove(VariableFlags::NEED_SYNC);
            self.flags.set(flags);

            // Set new value in a data model.
            (setter)(self.value.clone());

            true
        } else {
            false
        }
    }
}


impl<T> InheritableVariable<T> {
    /// Creates new modified variable from given value. This method should always be used to create inheritable
    /// variables in the engine.
    #[inline]
    pub fn new_modified(value: T) -> Self {
        Self {
            value,
            flags: Cell::new(VariableFlags::MODIFIED),
        }
    }

    /// Creates new variable without any flags set.
    #[inline]
    pub fn new_non_modified(value: T) -> Self {
        Self {
            value,
            flags: Cell::new(VariableFlags::NONE),
        }
    }

    /// Creates new variable from a given value with custom flags.
    #[inline]
    pub fn new_with_flags(value: T, flags: VariableFlags) -> Self {
        Self {
            value,
            flags: Cell::new(flags),
        }
    }

    /// Replaces value and also raises the [`VariableFlags::MODIFIED`] flag.
    #[inline]
    pub fn set_value_and_mark_modified(&mut self, value: T) -> T {
        self.mark_modified_and_need_sync();
        std::mem::replace(&mut self.value, value)
    }

    /// Replaces value and flags.
    #[inline]
    pub fn set_value_with_flags(&mut self, value: T, flags: VariableFlags) -> T {
        self.flags.set(flags);
        std::mem::replace(&mut self.value, value)
    }

    /// Replaces current value without marking the variable modified.
    #[inline]
    pub fn set_value_silent(&mut self, value: T) -> T {
        std::mem::replace(&mut self.value, value)
    }

    /// Returns true if the respective data model's variable must be synced.
    #[inline]
    pub fn need_sync(&self) -> bool {
        self.flags.get().contains(VariableFlags::NEED_SYNC)
    }

    /// Returns a reference to the wrapped value.
    #[inline]
    pub fn get_value_ref(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the wrapped value.
    ///
    /// # Important notes.
    ///
    /// The method raises `modified` flag, no matter if actual modification was made!
    #[inline]
    pub fn get_value_mut_and_mark_modified(&mut self) -> &mut T {
        self.mark_modified_and_need_sync();
        &mut self.value
    }

    /// Returns a mutable reference to the wrapped value.
    ///
    /// # Important notes.
    ///
    /// This method does not mark the value as modified!
    #[inline]
    pub fn get_value_mut_silent(&mut self) -> &mut T {
        &mut self.value
    }

    /// Returns true if variable was modified and should not be overwritten during property inheritance.
    #[inline]
    pub fn is_modified(&self) -> bool {
        self.flags.get().contains(VariableFlags::MODIFIED)
    }

    /// Marks value as modified, so its value won't be overwritten during property inheritance.
    #[inline]
    pub fn mark_modified(&mut self) {
        self.flags
            .get_mut()
            .insert(VariableFlags::MODIFIED | VariableFlags::NEED_SYNC);
    }

    /// Deconstructs the variable and returns the wrapped value.
    #[inline]
    pub fn take(self) -> T {
        self.value
    }

    #[inline]
    fn mark_modified_and_need_sync(&mut self) {
        self.flags
            .get_mut()
            .insert(VariableFlags::MODIFIED | VariableFlags::NEED_SYNC);
    }
}

impl<T> Deref for InheritableVariable<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for InheritableVariable<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mark_modified_and_need_sync();
        &mut self.value
    }
}

