This is a helper library which is dedicated to make extracting value from environment variables easy.

### How To Import

add below line to cargo.toml which is in your project

```toml
from-env = "*"

```

### Usage

```rust
use from_env::{FromEnv, FromEnvDerive}


#[derive(FromEnvDerive)]
struct MyServerConfig {
    log_level: Option<String>,
    listen_address: String,
    max_conns: i32,
    is_debug: bool,
}

fn main() {
    dotenv::dotenv().ok();
    let config = MyServerConfig::from_env();
    ...
}

```

### Support Data Types

almost all of primitive types in rust are supported, include:

- String
- i8
- i16
- i32
- i64
- i128
- u8
- u16
- u32
- u64
- u128
- usize
- isize
- bool
- f32
- f64
- Option<T: FromEnv>

you can also implement `FromEnv` for your type

### Environment Variable Name

If not specified, all environment variables name is the upper-case of field name:

```rust
struct MyConfig {
    my_var: String  // this will search for MY_VAR in environment varialbes
}
```

but you can use `#[env_var]` tag to specify which environment variable is related to the field:

```rust
struct MyConfig {
    #[env_var(THE_REAL_NAME)] // this will search for THE_REAL_NAME in environment variables
    my_var: String
}

```

### Optional Type Field

When environment variable whic you expected does not exist, `from_env` will panic if correspond field is not optional(i.e `Option<T>`). Optional field will be `None` if environment variable is not exists
