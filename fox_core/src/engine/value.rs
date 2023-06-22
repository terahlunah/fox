use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::engine::error::Error;

#[derive(Debug, Clone)]
pub struct Value(Rc<ValueData>);

impl From<ValueData> for Value {
    fn from(value: ValueData) -> Self {
        Value(Rc::new(value))
    }
}

impl Value {
    fn bool(b: bool) -> Value {
        ValueData::Bool(b).into()
    }

    fn char(c: char) -> Value {
        ValueData::Char(c).into()
    }

    fn num(n: f64) -> Value {
        ValueData::Num(n).into()
    }

    fn symbol(s: impl Into<String>) -> Value {
        ValueData::Symbol(s.into()).into()
    }

    fn native(o: Box<dyn NativeObject>) -> Value {
        ValueData::Native(o).into()
    }

    pub fn data_ref(&self) -> &ValueData {
        self.0.as_ref()
    }

    pub fn data_mut(&mut self) -> &mut ValueData {
        Rc::make_mut(&mut self.0)
    }

    pub fn as_bool(&self) -> Result<bool, Error> {
        match self.data_ref() {
            ValueData::Bool(b) => Ok(*b),
            _ => Err(Error::CastError("Bool".to_string())),
        }
    }

    pub fn as_char(&self) -> Result<char, Error> {
        match self.data_ref() {
            ValueData::Char(c) => Ok(*c),
            _ => Err(Error::CastError("Char".to_string())),
        }
    }

    pub fn as_num(&self) -> Result<f64, Error> {
        match self.data_ref() {
            ValueData::Num(n) => Ok(*n),
            _ => Err(Error::CastError("Num(".to_string())),
        }
    }

    pub fn as_symbol(&self) -> Result<&str, Error> {
        match self.data_ref() {
            ValueData::Symbol(s) => Ok(s.as_str()),
            _ => Err(Error::CastError("Symbol".to_string())),
        }
    }

    pub fn as_native(&self) -> Result<&dyn NativeObject, Error> {
        match self.data_ref() {
            ValueData::Native(o) => Ok(o.as_ref()),
            _ => Err(Error::CastError("Native".to_string())),
        }
    }

    pub fn as_native_mut(&mut self) -> Result<&mut dyn NativeObject, Error> {
        match self.data_mut() {
            ValueData::Native(o) => Ok(o.as_mut()),
            _ => Err(Error::CastError("Native".to_string())),
        }
    }

    pub fn as_native_typed<T: NativeObject>(&self) -> Result<&T, Error> {
        self.as_native()?
            .as_any()
            .downcast_ref::<T>()
            .ok_or(Error::CastError(std::any::type_name::<T>().to_string()))
    }

    pub fn as_native_typed_mut<T: NativeObject>(&mut self) -> Result<&mut T, Error> {
        self.as_native_mut()?
            .as_any_mut()
            .downcast_mut::<T>()
            .ok_or(Error::CastError(std::any::type_name::<T>().to_string()))
    }
}

pub trait AsAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Cast {
    fn cast_ref<T: 'static>(&self) -> Result<&T, Error>;
    fn cast_mut<T: 'static>(&mut self) -> Result<&mut T, Error>;
}

impl<A: AsAny> Cast for A {
    fn cast_ref<T: 'static>(&self) -> Result<&T, Error> {
        self.as_any()
            .downcast_ref::<T>()
            .ok_or(Error::CastError(std::any::type_name::<T>().to_string()))
    }

    fn cast_mut<T: 'static>(&mut self) -> Result<&mut T, Error> {
        self.as_any_mut()
            .downcast_mut::<T>()
            .ok_or(Error::CastError(std::any::type_name::<T>().to_string()))
    }
}

pub trait NativeObject: Debug + AsAny {
    fn repr(&self) -> String;
    fn cloned(&self) -> Box<dyn NativeObject>;
}

impl Clone for Box<dyn NativeObject> {
    fn clone(&self) -> Self {
        (*self).cloned()
    }
}

#[derive(Debug, Clone)]
pub enum ValueData {
    Bool(bool),
    Num(f64),
    Char(char),
    Symbol(String),
    Native(Box<dyn NativeObject>),
}

impl ValueData {
    pub fn repr(&self) -> String {
        match self {
            ValueData::Bool(v) => v.to_string(),
            ValueData::Num(v) => v.to_string(),
            ValueData::Char(v) => v.to_string(),
            ValueData::Symbol(v) => v.to_string(),
            ValueData::Native(v) => v.repr(),
        }
    }
}
