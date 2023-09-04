use std::fmt::{Display, Formatter};
use std::iter::Iterator;

#[derive(Debug, Clone)]
enum List<T> {
    Node {data: T, next: Box<List<T>> },
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

    fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter{ elm: self }
    }
}

struct ListIter<'a, T> {
    elm: &'a List<T>,
}
impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elm {
            List::Node {data, next} => {
                self.elm = next;
                Some(data)
            }
            List::Nil => None,
        }
    }
}

struct ImaginaryNumber {
    real: f64,
    img: f64,
}
impl Display for ImaginaryNumber {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"{} + {}i", self.real, self.img)
    }
}

fn main() {
    let n = ImaginaryNumber {real: 3.0, img:4.0};
    println!("{n}");

    let list = List::new().cons(0).cons(1).cons(2);

    for x in list.iter() {
        println!("{x}");
    }
    println!();

    let mut it = list.iter();

    println!("{:?}",it.next().unwrap());
    println!("{:?}",it.next().unwrap());
    println!("{:?}",it.next().unwrap());
}
