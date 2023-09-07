use ch_test::my_func;

// #[cfg(test)]
// mod test {
//     use super::*;
    
//     #[test]
//     fn test_my_func() {
//         assert_eq!(my_func(), Some(100));
//     }
// }

pub fn pred(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_pred() {
        pred(0).unwrap();
    }
}

fn main() {
    println!("Hello");
}