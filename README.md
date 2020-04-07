# no_error

A error library for no_std Rust

* macro transforms strings into C-strings
* no allocation required, static strings only
* specifying source is optional

```rust
use no_error::*;

extern "C" {
    fn print(x: const *u8);
}

fn can_fail() -> Result<()> {
    // programmatically appends a "/0" to end of static string
    error!("auto fail","failed in can_fail()")
}

fn can_fail_2() -> Result<()> {
    // no need to specify source
    error!("auto fail")
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
