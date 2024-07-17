//! Visitor is a tree-based serializer/deserializer.
//!
//! # Overview
//!
//! Visitor uses tree to create structured storage of data. Basic unit is a *node* - it is a container
//! for data fields. Each node has name, handle to parent, set of handles to children nodes and some
//! container for data fields. Data field is tuple of name and value, value can be any of simple Rust
//! types and some of basic structures of the crate. Main criteria of what could be the field and what
//! not is the ability to be represented as set of bytes without any aliasing issues.

pub use velcro_derive::Visit;

pub mod prelude {
    //! Types to use `#[derive(Visit)]`
    pub use super::{Visit, VisitError, VisitResult};
}

use bitflags::bitflags;
use std::fmt::{Display, Formatter};

use std::error::Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum VisitError {
    /// When a field is encoded as bytes, the field data is prefixed by an identifying byte
    /// to allow the bytes to be decoded. This error happens when an identifying byte is
    /// expected during decoding, but an unknown value is found in that byte.
    UnknownFieldType(u8),
    /// Attempting to visit a field on a read-mode Visitor when no field in the visitor data
    /// has the given name.
    FieldDoesNotExist(String),
    /// Attempting to visit a field on a write-mode Visitor when a field already has the
    /// given name.
    FieldAlreadyExists(String),
    /// Attempting to enter a region on a write-mode Visitor when a region already has the
    /// given name.
    RegionAlreadyExists(String),
    InvalidCurrentNode,
    /// Attempting to visit a field using a read-mode Visitor when when that field was originally
    /// written using a value of a different type.
    FieldTypeDoesNotMatch,
    /// Attempting to enter a region on a read-mode Visitor when no region in the visitor's data
    /// has the given name.
    RegionDoesNotExist(String),
    /// The Visitor tried to leave is current node, but somehow it had no current node. This should never happen.
    NoActiveNode,
    /// The [Visitor::MAGIC] bytes were missing from the beginning of encoded Visitor data.
    NotSupportedFormat,
    /// Some sequence of bytes was not in UTF8 format.
    InvalidName,
    /// Visitor data can be self-referential, such as when the data contains multiple [Rc] references
    /// to a single shared value. This causes the visitor to store the data once and then later references
    /// to the same value point back to its first occurrence. This error occurs if one of these references
    /// points to a value of the wrong type.
    TypeMismatch,
    /// Attempting to visit a mutably borrowed RefCell.
    RefCellAlreadyMutableBorrowed,
    /// A plain-text error message that could indicate almost anything.
    User(String),
    /// [Rc] and [Arc] values store an "Id" value in the Visitor data which is based in their internal pointer.
    /// This error indicates that while reading this data, one of those Id values was discovered by be 0.
    UnexpectedRcNullIndex,
    /// A poison error occurred while trying to visit a mutex.
    PoisonedMutex,
}

impl Error for VisitError {}


impl Display for VisitError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownFieldType(type_index) => write!(f, "unknown field type {}", type_index),
            Self::FieldDoesNotExist(name) => write!(f, "field does not exists {}", name),
            Self::FieldAlreadyExists(name) => write!(f, "field already exists {}", name),
            Self::RegionAlreadyExists(name) => write!(f, "region already exists {}", name),
            Self::InvalidCurrentNode => write!(f, "invalid current node"),
            Self::FieldTypeDoesNotMatch => write!(f, "field type does not match"),
            Self::RegionDoesNotExist(name) => write!(f, "region does not exists {}", name),
            Self::NoActiveNode => write!(f, "no active node"),
            Self::NotSupportedFormat => write!(f, "not supported format"),
            Self::InvalidName => write!(f, "invalid name"),
            Self::TypeMismatch => write!(f, "type mismatch"),
            Self::RefCellAlreadyMutableBorrowed => write!(f, "ref cell already mutable borrowed"),
            Self::User(msg) => write!(f, "user defined error: {}", msg),
            Self::UnexpectedRcNullIndex => write!(f, "unexpected rc null index"),
            Self::PoisonedMutex => write!(f, "attempt to lock poisoned mutex"),
        }
    }
}

impl<T> From<std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> for VisitError {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, T>>) -> Self {
        Self::PoisonedMutex
    }
}

impl<T> From<std::sync::PoisonError<&mut T>> for VisitError {
    fn from(_: std::sync::PoisonError<&mut T>) -> Self {
        Self::PoisonedMutex
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>> for VisitError {
    fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>) -> Self {
        Self::PoisonedMutex
    }
}


impl From<FromUtf8Error> for VisitError {
    fn from(_: FromUtf8Error) -> Self {
        Self::InvalidName
    }
}

impl From<String> for VisitError {
    fn from(s: String) -> Self {
        Self::User(s)
    }
}

/// The result of a [Visit::visit] or of a Visitor encoding operation
/// such as [Visitor::save_binary]. It has no value unless an error occurred.
pub type VisitResult = Result<(), VisitError>;

use velcro_utils::UUID;



bitflags! {
    /// Flags that can be used to influence the behaviour of [Visit::visit] methods.
    pub struct VisitorFlags: u32 {
        /// No flags set, do nothing special.
        const NONE = 0;
        /// Tell [crate::variable::InheritableVariable::visit] to assume that it's
        /// [VariableFlags::MODIFIED](create::variable::VariableFlags::MODIFIED) is set,
        /// and therefore write its data. Otherwise, InheritableVariable has the special
        /// property of *not writing itself* when the `MODIFIED` flag is not set.
        const SERIALIZE_EVERYTHING = 1 << 1;
    }
}