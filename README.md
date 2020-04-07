# no_error

An error library for no_std + no_alloc Rust

* macro transforms string literals into C style character array
* no allocator required
* support for error codes if C strings aren't an option

```rust
use no_error::*;

extern "C" {
    fn print(x: const *u8);
}

const FAIL:ErrorCode = 42;

fn can_fail(i:i32) -> Result<()> {
    if i < 3 { 
        // programmatically appends a "/0" to end of static string
        error_message!("auto fail","failed in can_fail()")
    } else {
        // don't like c strings? supports failure codes too
        error_code!(FAIL)
    }
}

fn can_fail_2() -> Result<()> {
    // no need to specify source if you don't want
    error_message!("auto fail")
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
