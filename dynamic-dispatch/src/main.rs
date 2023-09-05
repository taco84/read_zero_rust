use std::time;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct ErrorA;

impl Display for ErrorA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error A")
    }
}
impl Error for ErrorA {}

#[derive(Debug)]
struct ErrorB;

impl Display for ErrorB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error B")
    }
}
impl Error for ErrorB {}

fn _error_a() -> Result<(), ErrorA> {
    Err(ErrorA)
}

fn _error_b() -> Result<(), ErrorB> {
    Err(ErrorB)
}

fn _error_ab() -> Result<(), Box<dyn std::error::Error>> {
    _error_a()?;
    _error_b()?;
    Ok(())
}


trait Foo {
    fn foo(&self);
}

struct Bar;
impl Foo for Bar {
    fn foo(&self) {
        println!("Bar::foo");
    }
}

struct Buzz;
impl Foo for Buzz {
    fn foo(&self) {
        println!("Buzz::foo");
    }
}

fn call_foo_static<T: Foo>(arg: &T) {
    arg.foo();
}

fn call_foo_dynamic(arg: &dyn Foo) {
    arg.foo();
}

fn main() {
    let bar = Bar;
    let buzz = Buzz;

    let s_time = time::Instant::now();
    call_foo_static(&bar);
    call_foo_static(&buzz);
    println!("{:?}",s_time.elapsed());

    let d_time = time::Instant::now();
    call_foo_dynamic(&bar);
    call_foo_dynamic(&buzz);
    println!("{:?}",d_time.elapsed());
}
