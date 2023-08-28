use std::collections::LinkedList;

fn main() {
    let mut list1 = LinkedList::new();
    list1.push_back(0);
    list1.push_back(1);
    list1.push_back(2);

    let mut list2 = LinkedList::new();
    list2.push_back(100);
    list2.push_back(200);
    list2.push_back(300);

    list1.append(&mut list2);
    list1.push_front(-10);

    println!("{:?}",list1);
    println!("{:?}",list2);
}