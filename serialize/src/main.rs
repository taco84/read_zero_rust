use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}
impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    fn cons(self, data: T) -> List<T> {
        List::Node {
            data,
            next: Box::new(self),
        }
    }
}

fn main() {
    let list = List::new().cons(1).cons(2).cons(3);

    let js = serde_json::to_string(&list).unwrap();
    println!("JSON: {} bytes", js.len());
    println!("{js}");

    let yml = serde_yaml::to_string(&list).unwrap();
    println!("YAML: {} bytes",yml.len());
    println!("{yml}");

    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes",msgpack.len());

    let list = serde_json::from_str::<List<i32>>(&js).unwrap();
    println!("{:?}",list);

    let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    println!("{:?}",list);

    let list = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
    println!("{:?}",list);

    println!("");

    let list = List::new().cons(1).cons(2).cons(3);
    let yml = serde_yaml::to_string(&list).unwrap();

    let path = Path::new("test.yml");
    let mut f = File::create(path).unwrap();
    f.write_all(yml.as_bytes()).unwrap();

    let path = Path::new("test.yml");
    let mut f = File::open(path).unwrap();
    let mut yml = String::new();
    f.read_to_string(&mut yml).unwrap();

    let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    println!("{:?}",list);
}