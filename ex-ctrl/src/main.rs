use std::sync::{Arc,Mutex};
mod museum;

fn main() {
    let x = Arc::new(Mutex::new(100_000));
    let x2 = x.clone();

    let h1 = std::thread::spawn(move || {
        let mut guard = x.lock().unwrap();
        *guard -= 20_000;
    });

    let h2 = std::thread::spawn(move || {
        let mut guard = x2.lock().unwrap();
        *guard -= 30_000;
    });
    h1.join().unwrap();
    h2.join().unwrap();

    museum::run(); 
}
