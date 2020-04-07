# no_error

<a href="https://docs.rs/no_error"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

An error library for `no_std` + `no_alloc` Rust

* macro transforms string literals into C style character array
* no allocator required
* support for non-C string text (i.e. len + string)
* support for error codes if text isn't an option

```rust
use no_error::*;

extern "C" {
    fn print(x: const *u8);
}

const FAIL:ErrorCode = 42;

fn can_fail(i:i32) -> Result<()> {
    if i < 0 { 
        // programmatically appends a "/0" to end of static string
        error_message!("a failure happened","it happened in can_fail()")
    } else if i == 0 {
        // don't like c strings? supports failure codes too
        error_code!(FAIL)
    } else {
        // you don't have to specify the source if you don't want
        error_message!("a failure happened")
    }
}

fn main() {
    match can_fail() {
        Ok(_) => (),
        Err(a) => {
            print(a.cstr_description());
            print(a.cstr_source();
            if let Some(c) = a.code() {
                if c == FAIL {
                    print("secret of life".as_ptr());
                }
            }
        },
    };
}
```
