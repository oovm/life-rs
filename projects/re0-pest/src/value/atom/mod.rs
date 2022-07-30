use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Symbol(String),
    String(String),
    Integer(i64, String),
    Decimal(f64, String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Symbol(v) => write!(f, "{}", v),
            Value::Integer(v, s) => write!(f, "{}{}", v, s),
            Value::Decimal(v, s) => write!(f, "{}{}", v, s),
            Value::String(v) => write!(f, "{:?}", v),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<&str> for Value {
    fn from(key: &str) -> Self {
        Self::String(key.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, _) => {
                todo!()
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, _other: &Self) -> bool {
        todo!()
    }

    fn le(&self, _other: &Self) -> bool {
        todo!()
    }

    fn gt(&self, _other: &Self) -> bool {
        todo!()
    }

    fn ge(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Add<Self> for Value {
    type Output = Value;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Self> for Value {
    fn add_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl Sub<Self> for Value {
    type Output = Value;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Self> for Value {
    fn sub_assign(&mut self, _rhs: Self) {
        todo!()
    }
}
