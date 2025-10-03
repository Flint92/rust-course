fn main() {
    let x = 4;

    let equals_to_x = |y| y == x;

    let y = 4;
    assert!(equals_to_x(y));


    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();
    let ans = f(1);
    assert_eq!(ans, 6);

}
