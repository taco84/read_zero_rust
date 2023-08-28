fn main() {
    let v = vec![1,2,3,4];
    let mut result = 0;
    for x in v.iter() {
        if x % 2 == 0 {
            continue;
        }
        result += x;
    }
    println!("{}",result);
}