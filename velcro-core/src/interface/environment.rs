#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]


use std::{any::{Any, TypeId}, collections::HashMap, fmt::Error, ptr::NonNull, result, sync::Mutex};
use std::sync::Arc;
use std::rc::Rc;
use crate::parallel;

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

pub trait EnvironmentInterface {
    fn get_variable(&mut self, uid: &u32) -> EnvironmentVariableResult<Option<&Arc<dyn Any>>>;
}

pub struct Environment<T: EnvironmentInterface>  {
    _num_attached: u32,
    _fallback:  Option<NonNull<T>>,
    _variables: HashMap<u32, Box<dyn Any>>,
    _mutex:     Rc<Mutex<u32>>
}

impl<T: EnvironmentInterface>  EnvironmentInterface for Environment<T> {
    /*pub fn new() -> Self {
        Environment {
            _num_attached: 0,
            _fallback: None,
            _variables: HashMap::new(),
            _mutex: Rc::new(Mutex::new(0))
        }
    }*/


    /*pub fn remove_variable(&mut self, uid: &u32) -> EnvironmentVariableResult<Option<Arc<dyn Any>>> {
        let _locker = *self._mutex.lock().unwrap();
        let result = self._variables.remove(uid);
        if result.is_none() {
            return EnvironmentVariableResult {
                state: States::NotFound,
                variable: None,
            }
        }

        return EnvironmentVariableResult {
            state: States::Removed,
            variable: result,
        }
    }*/

    fn get_variable(&mut self, uid: &u32) -> EnvironmentVariableResult<Option<&Arc<dyn Any>>>{
        let result = self._variables.get(uid);
        match result {
            None => {
                match self._fallback {
                    None => return EnvironmentVariableResult {
                        state: States::NotFound,
                        variable: None,
                    },
                    Some(fallback) => {
                        return unsafe {
                         (*fallback.as_ptr()).get_variable(uid) };
                    }
                }
            },
            Some(res) => {

            }
        }

    }
}



/*pub trait EnvironmentInterface: 'static {
   fn attach_fallback(source_environment: Option<Arc<EnvironmentInterface>>);
   fn get_fallback() -> Option<Arc<EnvironmentInterface>>;
}*/

/*struct EnvironmentVariableHolderBase {
    _guid: u32,
    _use_count: int32,
    _mutex: parallel::SpinMutex,
}

pub struct EnvironmentVariable {

}

pub trait EnvironmentInterface {
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
