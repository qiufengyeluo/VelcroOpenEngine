use bitflags::bitflags;
use std::{
    any::{Any, TypeId},
    cell::Cell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

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