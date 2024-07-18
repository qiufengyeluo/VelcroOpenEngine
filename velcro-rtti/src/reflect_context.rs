//! Visitor is a tree-based serializer/deserializer.
//!
//! # Overview
//!
//! Visitor uses tree to create structured storage of data. Basic unit is a *node* - it is a container
//! for data fields. Each node has name, handle to parent, set of handles to children nodes and some
//! container for data fields. Data field is tuple of name and value, value can be any of simple Rust
//! types and some of basic structures of the crate. Main criteria of what could be the field and what
//! not is the ability to be represented as set of bytes without any aliasing issues.

pub use velcro_derive::Context;

pub mod prelude {
    //! Types to use `#[derive(Context)]`
    pub use super::{Context, ContextError, ReflectResult};
}




use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::{Display, Formatter};
use std::num;
use std::error::Error;
use std::io::{Read, Write};
use std::string::FromUtf8Error;

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
}


pub trait Pod: Copy {
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

pub struct Field {
    // 字段名字
    name: String,
    // 字段数据类型
    kind: FieldKind,
}


#[derive(Debug)]
pub enum ContextError {
    /// 未知得字段类型
    UnknownFieldType(u8),
    /// 字段不存在
    FieldDoesNotExist(String),
    /// 字段已存在
    FieldAlreadyExists(String),
    /// 在该区域已经存存在此名称
    RegionAlreadyExists(String),
    InvalidCurrentNode,
    /// 该字段的类型与定义的不相同
    FieldTypeDoesNotMatch,
    /// 该区域不存在指定名字
    RegionDoesNotExist(String),
    /// 访问者试图离开当前节点，但不知何故它没有当前节点.[这绝对不应该发生]
    NoActiveNode,
    /// 编码缺少[Visitor::MAGIC]字节数据
    NotSupportedFormat,
    /// 某些字节序列不是 UTF8 格式.
    InvalidName,
    /// 访问者数据可以是自引用的，例如当数据包含对单个共享值的多个 [Rc] 引用时. 这会导致访问者存储数据一次,
    /// 然后对相同值的引用返回到其第一次出现.如果这些引用之一指向错误类型的值，则会发生此错误.
    TypeMismatch,
    /// 尝试访问可变借用的 RefCell.
    RefCellAlreadyMutableBorrowed,
    /// 文本错误.
    User(String),
    /// [Rc] 和 [Arc] 值在访问者数据中存储基于其内部指针的“Id”值.  此错误表明在读取此数据时,发现这些 Id 值之一为 0.
    UnexpectedRcNullIndex,
    /// 尝试访问互斥体时发生错误.
    PoisonedMutex,
}

impl Error for ContextError {}


impl Display for ContextError {
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

impl<T> From<std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> for ContextError {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, T>>) -> Self {
        Self::PoisonedMutex
    }
}

impl<T> From<std::sync::PoisonError<&mut T>> for ContextError {
    fn from(_: std::sync::PoisonError<&mut T>) -> Self {
        Self::PoisonedMutex
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>> for ContextError {
    fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>) -> Self {
        Self::PoisonedMutex
    }
}


impl From<FromUtf8Error> for ContextError {
    fn from(_: FromUtf8Error) -> Self {
        Self::InvalidName
    }
}

impl From<String> for ContextError {
    fn from(s: String) -> Self {
        Self::User(s)
    }
}

/// The result of a [Context::context] or of a ReflectContext encoding operation
/// such as [ReflectContext::save_binary]. It has no value unless an error occurred.
pub type ReflectResult = Result<(), ContextError>;

use velcro_utils::UUID;
use crate::memory::Handle;

trait ElementaryField {
    fn write(&self, file: &mut dyn Write) -> ReflectResult;
    fn read(&mut self, file: &mut dyn Read) ->ReflectResult;
}

macro_rules! imp_elementary_field {
     ($ty:ty, $write:ident, $read:ident $(, $endian:ident)*) => {
         impl ElementaryField for $ty {
             fn write(&self, file: &mut dyn Write) -> ReflectResult {
                 file.$write::<$($endian)*>(*self)?;
                 Ok(())
             }
             
             fn read(&mut self, file: &mut dyn Read) -> ReflectResult {
                *self = file.$read::<$($endian)*>()?;
                 Ok(())
             }
         }
     };
}

imp_elementary_field!(f64, write_f64, read_f64, LittleEndian);
imp_elementary_field!(f32, write_f32, read_f32, LittleEndian);
imp_elementary_field!(u8, write_u8, read_u8);
imp_elementary_field!(i8, write_i8, read_i8);
imp_elementary_field!(u16, write_u16, read_u16, LittleEndian);
imp_elementary_field!(i16, write_i16, read_i16, LittleEndian);
imp_elementary_field!(u32, write_u32, read_u32, LittleEndian);
imp_elementary_field!(i32, write_i32, read_i32, LittleEndian);
imp_elementary_field!(u64, write_u64, read_u64, LittleEndian);
imp_elementary_field!(i64, write_i64, read_i64, LittleEndian);





bitflags! {
    /// Flags that can be used to influence the behaviour of [Context::context] methods.
     pub struct ContextFlags: u32 {
        /// No flags set, do nothing special.
        const NONE = 0;
        /// Tell [crate::variable::InheritableVariable::context] to assume that it's
        /// [VariableFlags::MODIFIED](create::variable::VariableFlags::MODIFIED) is set,
        /// and therefore write its data. Otherwise, InheritableVariable has the special
        /// property of *not writing itself* when the `MODIFIED` flag is not set.
        const SERIALIZE_EVERYTHING = 1 << 1;
     }
}

impl Field {
    pub fn new(name: &str, kind: FieldKind) -> Self {
        Field {
            name: name.to_owned(),
            kind,
        }
    }

    fn save(field: &Field, file: &mut dyn Write) -> ReflectResult {
        let name = field.name.as_bytes();
        file.write_u32::<LittleEndian>(name.len() as u32)?;
        file.write_all(name)?;
        match &field.kind {
            FieldKind::U8(data) => {
                file.write_u8(1)?;
                file.write_u8(*data)?;
            }
            FieldKind::I8(data) => {
                file.write_i8(2)?;
                file.write_i8(*data)?;
            }
            FieldKind::U16(data) => {
                file.write_u8(3)?;
                file.write_u16::<LittleEndian>(*data)?;
            }
            FieldKind::I16(data) => {
                file.write_u8(4)?;
                file.write_i16::<LittleEndian>(*data)?;
            }
            FieldKind::U32(data) => {
                file.write_u8(5)?;
                file.write_u32::<LittleEndian>(*data)?;
            }
            FieldKind::I32(data) => {
                file.write_u8(6)?;
                file.write_i32::<LittleEndian>(*data)?;
            }
            FieldKind::U64(data) => {
                file.write_u8(7)?;
                file.write_u64::<LittleEndian>(*data)?;
            }
            FieldKind::I64(data) => {
                file.write_u8(8)?;
                file.write_i64::<LittleEndian>(*data)?;
            }
            FieldKind::F32(data) => {
                file.write_u8(9)?;
                file.write_f32::<LittleEndian>(*data)?;
            }
            FieldKind::F64(data) => {
                file.write_u8(10)?;
                file.write_f64::<LittleEndian>(*data)?;
            }
            FieldKind::BinaryBlob(data) => {
                file.write_u8(14)?;
                file.write_u32::<LittleEndian>(data.len() as u32)?;
                file.write_all(data.as_slice())?;
            }
            FieldKind::Bool(data) => {
                file.write_u8(15)?;
                file.write_u8(u8::from(*data))?;
            }
        }
        Ok(())
    }

    fn load(file: &mut dyn Read) -> Result<Field, ContextError> {
        let name_len = file.read_u3::<LittleEndian>()? as usize;
        let mut raw_name = vec![Default::default(); name_len];
        file.read_exact(raw_name.as_mut_slice())?;
        let id = file.read_u8()?;
        Ok(Field::new(String::from_utf8(raw_name)?.as_str(),
        match id {
            1 => FieldKind::U8(file.read_u8()?),
            2 => FieldKind::I8(file.read_i8()?),
        }))
    }

}

pub struct ContextNode {
    name: String,
    fields: Vec<Field>,
    parent: Handle<ContextNode>,
    children: Vec<Handle<ContextNode>>,
}

impl ContextNode {
    fn new(name: &str, parent: Handle<ContextNode>) -> Self {
        Self {
            name: name.to_owned(),
            fields: Vec::new(),
            parent,
            children: Vec::new(),
        }
    }
}


pub struct ReflectContext {
    pub flags: ContextFlags,
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        let a: u32  = 1;
    }
}