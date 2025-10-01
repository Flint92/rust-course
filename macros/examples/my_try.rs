// ? operator. How to simulate it
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}

fn main() -> Result<(), String> {

    let r = my_try!(f1(my_try!(f2())));
    println!("{:?}", r);

    Ok(())
}

fn f1(s: String) -> Result<String, String> {
    Ok(format!("f1: {}", s))
}

fn f2() -> Result<String, String> {
    Err(format!("error opening file: {}", "hello.txt"))
}