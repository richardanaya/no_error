use no_error::*;

const FAIL: ErrorCode = 0;

fn can_fail(i: i32) -> Result<()> {
    if i < 3 {
        error_message!("foo")
    } else {
        error_code!(FAIL)
    }
}

fn main() {
    match can_fail(42) {
        Ok(_) => println!("ok"),
        Err(a) => println!("{} {}", a.description(), a.source()),
    };
}
