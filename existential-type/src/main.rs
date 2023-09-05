use std::any::{Any, TypeId};

fn closure() -> impl Fn(i32) -> i32 {
    |x| x * x
}

fn get_type_id<T: Any>(_: &T) -> TypeId {
    TypeId::of::<T>()
}

fn main() {
    let f = closure();
    f(10);
    
    let f = |x: i32| x * x;
    let g = |x: i32| x * x;

    println!("{}", get_type_id(&f) == get_type_id(&g));
}