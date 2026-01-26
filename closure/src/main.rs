fn main() {
    let x = 4;

    let equals_to_x = |y| y == x;

    let y = 4;
    assert!(equals_to_x(y));


    let double_it = |n| n * 2;
    dbg!(double_it(1));

    let add_one = |n: i32| -> i32 { n + 1 };
    dbg!(add_one(5));

    let max_value = 5;

    let clamp = |v| {
        if v > max_value { max_value } else { v }
    };

    dbg!(clamp(1));
    dbg!(clamp(3));
    dbg!(clamp(5));
    dbg!(clamp(7));


    let suffix = "-itis";
    let add_suffix = |x| format!("{x}{suffix}");
    apply_and_log(&add_suffix, "add_suffix", "senior");
    apply_and_log(&add_suffix, "add_suffix", "appendix");
    let mut v = Vec::new();
    let mut accumulate = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut accumulate, "accumulate", "red");
    apply_and_log(&mut accumulate, "accumulate", "green");
    apply_and_log(&mut accumulate, "accumulate", "blue");
    let take_and_reverse = |prefix| {
        let mut acc = String::from(prefix);
        acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        acc
    };
    apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");


    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();
    let ans = f(1);
    assert_eq!(ans, 6);

}

fn apply_and_log(
    func: impl FnOnce(&'static str) -> String,
    func_name: &'static str,
    input: &'static str,
) {
    println!("Calling {func_name}({input}): {}", func(input))
}
