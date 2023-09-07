use my_great_lib;

#[test]
fn test_rand_even() {
    for _ in 0..100 {
        let result = my_great_lib::rand_even();
        asert_eq!(result % 2, 0);
    }
}
