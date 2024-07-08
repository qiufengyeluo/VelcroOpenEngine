#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]


use std::{any::{Any, TypeId}, collections, fmt::Error};

pub enum States {
    Added,
    Removed,
    Found,
    NotFound,
    OutOfMemory,
}

pub struct EnvironmentVariableResult<T> {
    pub state: States,
    pub variable: T
}

impl<T> EnvironmentVariableResult<T> {
    pub fn new(s: States, v: T) -> Self {
        EnvironmentVariableResult {
            state: s,
            variable: v
        }
    }
}

struct EnvironmentVariableHolderBase {
    _guid: u32,
}

/*pub struct EnvironmentVariable {

}*/

/*pub trait EnvironmentInterface {
    type Item;

    fn attach_fallback(&self, source_environment: Box<dyn EnvironmentInterface<Item = Self::Item>> );

    fn detach_fallback(&self);

    fn get_fallback(&self) -> Box<dyn EnvironmentInterface<Item = Self::Item>>;
    
    fn find_variable(&self, id: u32) -> Option<Self::Item>;
    fn remove_variable(&self, id: u32) -> EnvironmentVariableResult<Self::Item>;
    fn get_variable(&self, id: u32) -> EnvironmentVariableResult<Self::Item>;
}*/

/*type MapType = collections::HashMap<u32, Box<dyn Any>>;

pub struct Environment {
    _variable_map: MapType
}


impl Environment {
    pub fn remove_variable(&mut self, id: &u32) -> EnvironmentVariableResult<Option<Box<dyn Any>>> {
       let result: Option<Box<dyn Any>> = self._variable_map.remove(id);
       if result.is_none() {
        return EnvironmentVariableResult::new(States::NotFound, None);
       }

       return EnvironmentVariableResult::new(States::Removed, result);
    }

    pub fn get_variable(&self, id: &u32) -> EnvironmentVariableResult<Option<&Box<dyn Any>>>{
        let result: Option<&Box<dyn Any>> = self._variable_map.get(id);
        if result.is_none() {
            return EnvironmentVariableResult::new(States::NotFound, None);
        }

        return EnvironmentVariableResult::new(States::Found, result);
    }
}*/
