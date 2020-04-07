# no_error

An error library for no_std Rust

* no allocator required
* macro transforms strings into C-strings
* no allocation required, static strings only
* specifying source is optional

```rust
use no_error::*;

extern "C" {
    fn print(x: const *u8);
}

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
            print(a.description().as_ptr())
            print(a.source().as_ptr())
        },
    };
}
```