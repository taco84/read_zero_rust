use std::collections::{BTreeMap,BTreeSet, LinkedList};

fn main() {
    let mut m = BTreeMap::new();

    m.insert(1, "apple");
    m.insert(2, "orange");
    m.insert(3, "banana");

    if let Some(old) = m.remove(&2) {
        println!("{old}");
    }

    if let Some(value) = m.get(&3) {
        println!("{value}");
    }

    let mut s = BTreeSet::new();
    s.insert(400);
    s.insert(100);
    s.insert(500);
    s.insert(2);

    for n in s.iter() {
        println!("{}",n);
    }

    let mut v = Vec::new();

    v.push(100);
    v.push(400);

    let it = v.iter().chain(s.iter());
    for n in it.clone().map(|n| n * 2) {
        println!("{}",n);
    }

    let _total = it.clone().fold(0,|acc,x| acc + x);

    let _list: LinkedList<_> = it.filter(|n| *n%2 == 0).collect();

    for(n,m) in v.iter().zip(s.iter()) {
        println!("{} {}", n, m);
    }
}