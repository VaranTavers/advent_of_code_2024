// const LOG: bool = false;

#[macro_export]
macro_rules! log {
    ($($x:tt)*) => { if LOG { print!($($x)*) } }
}

#[macro_export]
macro_rules! logln {
    ($($x:tt)*) => { if LOG { println!($($x)*) } }
}
