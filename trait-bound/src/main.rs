use std::{ sync::Arc, thread};

fn main() {
    let n = Arc::new(10);
    thread::spawn(move ||{
        println!("{n}");
    });
}