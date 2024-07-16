#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]


use std::{any::Any, collections::HashMap, ptr::NonNull, sync::{Mutex, Once}, mem::MaybeUninit};
use std::rc::Rc;
//use crate::parallel;

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

pub trait EnvironmentInterface  {
    fn get_lock(&self) -> Rc<Mutex<u32>>;

    fn attach_fallback(&mut self,source_environment: Option<NonNull<dyn EnvironmentInterface>>);

    fn detach_fallback(&mut self);

    fn remove_variable(&mut self, uid: &u32) -> EnvironmentVariableResult<Option<Box<dyn Any>>>;

    fn find_variable(&self, uid: &u32) ->Option<&Box<dyn Any>>;
    
    fn get_variable(&self, uid: &u32) -> EnvironmentVariableResult<Option<&Box<dyn Any>>>;
}

pub struct Environment {
    _variables: HashMap<u32, Box<dyn Any>>,
    _mutex:     Rc<Mutex<u32>>,
    _fallback:  Option<NonNull<dyn EnvironmentInterface>>,
}

pub struct EnvironmentSingleton {
   inner: Mutex<Option<NonNull<dyn EnvironmentInterface>>>
}



impl  Environment  {
    pub fn new() -> Self {
        Environment {
            _fallback: None,
            _variables: HashMap::new(),
            _mutex: Rc::new(Mutex::new(0))
        }
    }

    pub fn get() -> &'static EnvironmentSingleton {
       static mut ENVIRONMENT_SINGLETON: MaybeUninit<EnvironmentSingleton> = MaybeUninit::uninit();
       static ENVIRONMENT_SINGLETON_ONCE: Once = Once::new();

       unsafe {
        ENVIRONMENT_SINGLETON_ONCE.call_once(|| {
            let _enviroment_singleton = EnvironmentSingleton {
                inner: Mutex::new(Some((NonNull::<Environment>::new(&mut Environment::new())).unwrap().cast::<Environment>() as  NonNull<dyn EnvironmentInterface>))
            };
            ENVIRONMENT_SINGLETON.write(_enviroment_singleton);
        });

        ENVIRONMENT_SINGLETON.assume_init_ref()
       }
    }

    pub fn attach(source_environment: Option<NonNull<dyn EnvironmentInterface>>, use_as_get_fallback: bool) {
        match  source_environment {
            None => {

            },
            Some(res) => {
                
                Environment::detach();

                {
                    // 两个生成周期必须同步
                    let ctrl = unsafe {source_environment.unwrap().as_ref()}.get_lock(); 
                    let _ctrl_locker =  ctrl.lock().unwrap();

                    if use_as_get_fallback {
                        unsafe { (*Environment::get().inner.lock().unwrap()).unwrap().as_mut().attach_fallback(Some(res))  };
                    } else {
                        let mut environment_ptr  = Environment::get().inner.lock().unwrap();
                        *environment_ptr = Some(res);
                        unsafe { (*environment_ptr).unwrap().as_mut().attach_fallback(None) };
                    }
                }
            }
        }
    }

    pub fn detach() {

    }
}

impl EnvironmentInterface for Environment {
    fn get_lock(&self) -> Rc<Mutex<u32>> {
        return Rc::clone(&self._mutex);
    }


    fn attach_fallback(&mut self,source_environment: Option<NonNull<dyn EnvironmentInterface>>) {
        self._fallback = source_environment;
    }

    fn detach_fallback(&mut self) {
        match self._fallback {
            None => {}
            Some(..) => {
                self._fallback = None;
            }
        }
    }

    fn remove_variable(&mut self, uid: &u32) -> EnvironmentVariableResult<Option<Box<dyn Any>>> {
        let _locker = *self._mutex.lock().unwrap();
        let result = self._variables.remove(uid);
        match result {
            None => {
                return EnvironmentVariableResult {
                    state: States::NotFound,
                    variable: None,
                }
            },
            Some(object) => {
                return EnvironmentVariableResult {
                    state: States::Removed,
                    variable: Some(object),
                }
            }
        }
    }

    fn find_variable(&self, uid: &u32) ->Option<&Box<dyn Any>> {
        let _locker = *self._mutex.lock().unwrap();
        return self._variables.get(uid);
    }

    fn get_variable(&self, uid: &u32) -> EnvironmentVariableResult<Option<&Box<dyn Any>>>{
        let _locker = *self._mutex.lock().unwrap();
        let result = self._variables.get(uid);
        match result {
            None => {
                match self._fallback {
                    None => return EnvironmentVariableResult {
                        state: States::NotFound,
                        variable: None,
                    },
                    Some(fallback) => {
                        return unsafe { (*fallback.as_ptr()).get_variable(uid) };
                    }
                }
            },
            Some(res) => {
                return EnvironmentVariableResult {
                    state: States::Found,
                    variable: Some(res),
                }
            }
        }
    }
}
