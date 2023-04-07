fn main() {
    println!("Hello, world!");
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    assert_eq!(2 + 5, 7);
    // code that takes an hour to run
}
