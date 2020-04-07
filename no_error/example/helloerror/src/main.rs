use no_error::*;

fn can_fail() -> Result<(), impl Error> {
    error!("foo")
}

fn main() {
    match can_fail() {
        Ok(_) => println!("ok"),
        Err(a) => println!("{}", a.description()),
    };
}
