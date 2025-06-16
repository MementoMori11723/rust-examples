// The main function
fn main() {
    println!("Hello, world!"); //this is short-cut or a macro that allows us to easily print data!
    _variables();
    _loop();
    _object();
}

// A function to show how variables are used
fn _variables() {
    let x = 10; // cannot change this variable!
    let mut y = x + 20; // can change this variable!
    y += 30;
    let z = &y; // this is a reference
    println!("{}", z);
}

// A simple loop function
fn _loop() {
    let mut count = 0;
    loop {
        count += 1;

        if count >= 10 {
            break;
        }

        if count == 5 {
            println!("Skipping 5");
            continue;
        }

        println!("Count is {}", count);
    }
}

// A function to create an object
fn _object() {
    struct Person {
        name: String,
        age: u32,
        is_male: bool,
    }

    impl Person {
        fn print(&self) {
            println!("Hi, Your name is {}.", self.name);
            println!("Your age is {}", self.age);
            match self.is_male {
                true => println!("You are Male!"),
                false => println!("You are Female!"),
            }
        }
    }

    Person{
        name: String::from("Bob"),
        age: 32,
        is_male: true,
    }.print();

    Person{
        name: String::from("Emily"),
        age: 18,
        is_male: false,
    }.print();
}
