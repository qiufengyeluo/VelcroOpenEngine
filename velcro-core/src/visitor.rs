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
    //pub use super::{Visit, VisitError, VisitResult, Visitor};
}

use velcro_utils::base64;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::any::TypeId;
use std::error::Error;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Display, Formatter},
    fs::File,
    hash::{BuildHasher, Hash},
    io::{BufWriter, Cursor, Read, Write},
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
    rc::Rc,
    string::FromUtf8Error,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};
use velcro_utils::UUID;
pub enum FieldKind {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    BinaryBlob(Vec<u8>),
    UUID(UUID),
    /// A representation for arrays of [Pod] types as a `Vec<u8>`.
    PodArray {
        /// A code to identify the Pod type of the elements of the array.
        /// Taken from [Pod::type_id].
        type_id: u8,
        /// The number of bytes in each array element.
        element_size: u32,
        /// The bytes that store the data, with unspecified endianness.
        bytes: Vec<u8>,
    },
}

/// Trait for datatypes that can be converted directly into bytes.
/// This is required for the type to be used in the Vec of a [PodVecView].
pub trait Pod: Copy {
    /// A number which distinguishes each Pod type. Two distinct Pod types must not share the same `type_id` byte.
    /// The `type_id` is stored with the data when a [PodVecView] is visited and used to confirm that the stored
    /// data matches the expected type when reading. Otherwise garbage data could be read by interpreting an
    /// array of i8 as an array of f32 or any other mismatched combination.
    fn type_id() -> u8;
}

impl Pod for u8 {
    fn type_id() -> u8 {
        0
    }
}

impl Pod for i8 {
    fn type_id() -> u8 {
        1
    }
}

impl Pod for u16 {
    fn type_id() -> u8 {
        2
    }
}

impl Pod for i16 {
    fn type_id() -> u8 {
        3
    }
}

impl Pod for u32 {
    fn type_id() -> u8 {
        4
    }
}

impl Pod for i32 {
    fn type_id() -> u8 {
        5
    }
}

impl Pod for u64 {
    fn type_id() -> u8 {
        6
    }
}

impl Pod for i64 {
    fn type_id() -> u8 {
        7
    }
}

impl Pod for f32 {
    fn type_id() -> u8 {
        8
    }
}

impl Pod for f64 {
    fn type_id() -> u8 {
        9
    }
}

/// A [Visit] type for storing a whole Vec of [Pod] values as a single field within a Visitor.
/// The Vec is reinterpreted as a Vec of bytes, with no consideration given for whether the bytes
/// are in big-endian or little-endian order by using [std::ptr::copy_nonoverlapping].
pub struct PodVecView<'a, T: Pod> {
    type_id: u8,
    vec: &'a mut Vec<T>,
}


impl<'a, T: Pod> PodVecView<'a, T> {
    pub fn from_pod_vec(vec: &'a mut Vec<T>) -> Self {
        Self {
            type_id: T::type_id(),
            vec,
        }
    }
}

/*impl<'a, T: Pod> Visit for PodVecView<'a, T> {
    #[allow(clippy::uninit_vec)]
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        if visitor.reading {
            if let Some(field) = visitor.find_field(name) {
                match &field.kind {
                    FieldKind::PodArray {
                        type_id,
                        element_size,
                        bytes,
                    } => {
                        if *type_id == self.type_id {
                            let len = bytes.len() / *element_size as usize;
                            let mut data = Vec::<T>::with_capacity(len);
                            unsafe {
                                data.set_len(len);
                                std::ptr::copy_nonoverlapping(
                                    bytes.as_ptr(),
                                    data.as_mut_ptr() as *mut u8,
                                    bytes.len(),
                                );
                            }
                            *self.vec = data;
                            Ok(())
                        } else {
                            Err(VisitError::TypeMismatch)
                        }
                    }
                    _ => Err(VisitError::FieldTypeDoesNotMatch),
                }
            } else {
                Err(VisitError::FieldDoesNotExist(name.to_owned()))
            }
        } else if visitor.find_field(name).is_some() {
            Err(VisitError::FieldAlreadyExists(name.to_owned()))
        } else {
            let node = visitor.current_node();
            node.fields.push(Field::new(
                name,
                FieldKind::PodArray {
                    type_id: T::type_id(),
                    element_size: std::mem::size_of::<T>() as u32,
                    bytes: unsafe {
                        let mut data = self.vec.clone();
                        let bytes = Vec::from_raw_parts(
                            data.as_mut_ptr() as *mut u8,
                            data.len() * std::mem::size_of::<T>(),
                            data.capacity() * std::mem::size_of::<T>(),
                        );
                        std::mem::forget(data);
                        bytes
                    },
                },
            ));
            Ok(())
        }
    }
}


/// Values within a visitor are constructed from Fields.
/// Each Field has a name and a value. The name is used as a key to access the value
/// within the visitor using the [Visit::visit] method, so each field within a value
/// must have a unique name.
pub struct Field {
    /// The key string that allows access to the field.
    name: String,
    /// The data stored in the visitor for this field.
    kind: FieldKind,
}

/// Errors that may occur while reading or writing [Visitor].
#[derive(Debug)]
pub enum VisitError {
    /// An [std::io::Error] occured while reading or writing a file with Visitor data.
    Io(std::io::Error),
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
            Self::Io(io) => write!(f, "io error: {}", io),
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
            Self::FileLoadError(e) => write!(f, "file load error: {:?}", e),
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

impl From<std::io::Error> for VisitError {
    fn from(io_err: std::io::Error) -> Self {
        Self::Io(io_err)
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

trait VisitableElementaryField {
    fn write(&self, file: &mut dyn Write) -> VisitResult;
    fn read(&mut self, file: &mut dyn Read) -> VisitResult;
}


/// A node is a collection of [Fields](Field) that exists within a tree of nodes
/// that allows a [Visitor] to store its data.
/// Each node has a name, and may have a parent node and child nodes.
pub struct VisitorNode {
    name: String,
    fields: Vec<Field>,
    parent: Handle<VisitorNode>,
    children: Vec<Handle<VisitorNode>>,
}

impl VisitorNode {
    fn new(name: &str, parent: Handle<VisitorNode>) -> Self {
        Self {
            name: name.to_owned(),
            fields: Vec::new(),
            parent,
            children: Vec::new(),
        }
    }
}

impl Default for VisitorNode {
    fn default() -> Self {
        Self {
            name: String::new(),
            fields: Vec::new(),
            parent: Handle::NONE,
            children: Vec::new(),
        }
    }
}*/