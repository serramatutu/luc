//! non-verbose match statement that returns a custom error
macro_rules! ok {
    ($expr: expr, $err: expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => return Err($err),
        }
    };
}
