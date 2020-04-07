#![no_std]

trait NoError {
    fn description(&self) -> &'static str;
    fn source(&self) -> &'static str;
}

impl NoError for (&'static str,&'static str) {
    fn description(&self) -> &'static str {
        self.0
    }

    fn source(&self) -> &'static str {
        self.1
    }
}

#[macro_export]
macro_rules! error {
    ($desc: tt, $source: tt) => {
        Err(($desc, $source))
    };
}
