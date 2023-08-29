use std::thread;
use std::sync::mpsc;


fn worker() -> u32 {
    println!("worker");
    100
}

struct XOR64 {
    x: u64,
}

impl XOR64 {
    fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88172645463325252,
        }
    }

    fn next(&mut self) -> u64 {
        let x = self.x;
        let x = x ^ (x << 13);
        let x = x ^ (x >> 7);
        let x = x ^ (x << 17);
        self.x = x;
        return x;
    }
}

const NUM: usize = 200000000;

fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    let mut generator = XOR64::new(1234);

    for _ in 0..NUM {
        let x1 = generator.next();
        let x2 = generator.next();
        v1.push(x1);
        v2.push(x2);
    }
    (v1,v2)
}

fn single_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now();

    v1.sort();
    v2.sort();

    let end = start.elapsed();
    println!(
        "single_thread: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos()
    );
}

fn multi_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now();

    let handler1 = thread::spawn(move || {
        v1.sort();
        v1
    });
    let handler2 = thread::spawn(move || {
        v2.sort();
        v2
    });

    let _v1 = handler1.join().unwrap();
    let _v2 = handler2.join().unwrap();
    let end = start.elapsed();

    println!(
        "multi_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos()
    );
}

fn main() {
    let handler = thread::spawn(worker);

    match handler.join() {
        Ok(n) => println!("{}",n),
        Err(e) => println!("{:?}",e),
    }

    //複数スレッドから受信したい場合はcrossbeam_channelというライブラリを使う
    //複数スレッドから送信したい場合はtxをcloneする

    let (tx,rx) = mpsc::channel();

    let handler = thread::spawn(move || match rx.recv() {
        Ok((x,y)) => println!("{}, {}",x, y),
        Err(e) => eprintln!("{}",e),
    });

    if let Err(e) = tx.send((10,20)) {
        eprintln!("{}",e);
    }

    if let Err(e) = handler.join() {
        eprintln!("{:?}",e);
    }

    single_threaded();
    multi_threaded();

}