use std::env;
use std::str::FromStr;

pub trait FromEnv
where
    Self: Sized,
{
    fn from_env(name: &str) -> Self;
}

impl FromEnv for String {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for i8 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for i16 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for i32 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for i64 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for i128 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for u8 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for u16 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for u32 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for u64 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for u128 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for usize {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for isize {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for bool {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for f32 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl FromEnv for f64 {
    fn from_env(name: &str) -> Self {
        Self::from_str(&env::var(name).unwrap()).unwrap()
    }
}

impl<T> FromEnv for Option<T>
where
    T: FromEnv,
{
    fn from_env(name: &str) -> Self {
        if let Ok(value) = env::var(name) {
            return Some(T::from_env(&value));
        }
        None
    }
}
