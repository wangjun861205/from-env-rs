use core::panic;
/// This is a helper library which is dedicated to make extracting value from environment variables easy.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nb_from_env::{FromEnv, FromEnvDerive};
///
/// #[derive(FromEnvDerive)]
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
/// use nb_from_env::{FromEnv, FromEnvDerive};
///
/// #[derive(FromEnvDerive)]
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
/// use nb_from_env::{FromEnv, FromEnvDerive};
///
/// #[derive(FromEnvDerive)]
/// struct TestStruct {
///    #[env_var(FULL_NAME)]
///    name: String,
/// }
/// std::env::set_var("FULL_NAME", "John");
///
/// let s = TestStruct::from_env();
/// println!("{}", s.name); // Prints "John"
/// ```
use std::str::FromStr;

pub use from_env_derive::FromEnvDerive;

pub trait FromEnv
where
    Self: Sized,
{
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self;
}

impl FromEnv for String {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return v;
        }
        default.unwrap_or_else(|| panic!("{} is required", env_var))
    }
}

impl FromEnv for i8 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid i8 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for i16 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid i16 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for i32 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid i32 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl FromEnv for i64 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid i64 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for i128 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid i128 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for u8 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid u8 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for u16 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid u16 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for u32 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid u32 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl FromEnv for u64 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid u64 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl FromEnv for u128 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid u128 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl FromEnv for usize {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid usize literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl FromEnv for isize {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid isize literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for bool {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid bool literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var)
    }
}

impl FromEnv for f32 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid f32 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl FromEnv for f64 {
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if let Some(v) = val {
            return Self::from_str(&v)
                .unwrap_or_else(|e| panic!("{} is not a valid f64 literal: {}", v, e));
        }
        if let Some(value) = default {
            return Self::from_str(&value).unwrap_or_else(|e| {
                panic!("failed to parse {}(value: {}): {}", env_var, value, e)
            });
        }
        panic!("{} default value is not existing", env_var);
    }
}

impl<T> FromEnv for Option<T>
where
    T: FromEnv,
{
    fn from_env(env_var: &str, val: Option<String>, default: Option<String>) -> Self {
        if val.is_none() && default.is_none() {
            return None;
        }
        Some(T::from_env(env_var, val, default))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(FromEnvDerive)]
    struct TestStruct {
        name: String,
        age: i32,
        #[env_var(MOBILE)]
        phone: String,
        #[env_default("unknown")]
        address: Option<String>,
        married: Option<bool>,
        credit_card: Option<String>,
        #[env_var(GENDER)]
        #[env_default("famale")]
        sex: String,
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
        assert!(test_struct.address == Some("unknown".to_string()));
        assert!(test_struct.married.unwrap());
        assert!(test_struct.credit_card.is_none());
        assert!(test_struct.sex == "famale");

        std::env::set_var("GENDER", "male");
        let test_struct = TestStruct::from_env();
        assert!(test_struct.sex == "male");
    }
}
