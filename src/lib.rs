/// This is a helper library which is dedicated to make extracting value from environment variables easy.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// struct TestStruct {
///    name: String,
///    age: i32,
/// }
/// std::env::set_var("NAME", "John");
/// std::env::set_var("AGE", "20");
///
/// let s = TestStruct::from_env();
/// println!("{}", s.name); // Prints "John"
/// println!("{}", s.age); // Prints 20
/// ```
///
/// With optional values:
///
/// ```
/// struct TestStruct {
///    name: String,
///    age: Option<i32>,
/// }
/// std::env::set_var("NAME", "John");
///
/// let s = TestStruct::from_env();
/// println!("{}", s.name); // Prints "John"
/// println!("{:?}", s.age); // Prints None
/// ```
///
/// Specify environment variable name:
///
/// ```
/// struct TestStruct {
///    #[env_var(FULL_NAME)]
///    name: String,
/// }
/// std::env::set_var("FULL_NAME", "John");
///
/// let s = TestStruct::from_env();
/// println!("{}", s.name); // Prints "John"
/// ```
use std::env;
use std::str::FromStr;

pub use from_env_derive::FromEnvDerive;

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
        if env::var(name).is_ok() {
            return Some(T::from_env(name));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(FromEnvDerive)]
    struct TestStruct {
        pub name: String,
        pub age: i32,
        #[env_var(MOBILE)]
        pub phone: String,
        pub address: Option<String>,
        pub married: Option<bool>,
    }

    #[test]
    fn test_from_env() {
        std::env::set_var("NAME", "bob");
        std::env::set_var("AGE", "30");
        std::env::set_var("MOBILE", "123456");
        std::env::set_var("MARRIED", "true");

        let test_struct = TestStruct::from_env();
        assert!(test_struct.name == "bob");
        assert!(test_struct.age == 30);
        assert!(test_struct.phone == "123456");
        assert!(test_struct.address.is_none());
        assert!(test_struct.married.unwrap());
    }
}
