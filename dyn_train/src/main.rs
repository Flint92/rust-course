#[derive(Debug)]
struct Dog {
    name: String,
    age: i8,
}

#[derive(Debug)]
struct Cat {
    lives: i8
}

trait Pet {
    fn talk(&self) -> String;
}


impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, I am {} and I am {} years old", self.name, self.age)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Meow, I have {} lives", self.lives)
    }
}

// Uses generics and static dispatch.
fn generic(pet: &impl Pet) {
    dbg!(pet.talk());
}

// Uses type-erasure and dynamic dispatch.
fn dynamic(pet: &dyn Pet) {
    dbg!(pet.talk());
}


fn main() {
    let cat = Cat { lives: 9 };
    let dog = Dog { name: String::from("Fido"), age: 5 };

    generic(&cat);
    generic(&dog);

    dynamic(&cat);
    dynamic(&dog);
}
