// when pass a reference to a function with & , the ownership is not transferred.
// the reference is moved into the function, but the value is not moved.
// at the end of the function, the reference is dropped, but the value is not dropped.
// under the hood, the reference is a pointer to the value.
// you can read the value by dereferencing with * .

// when pass a mutable reference to a function with &mut ,
// you can read and write the value after dereferencing with * .

// the rules for ownership and references are:
// 1. at any given time, you can have either one mutable reference or any number of immutable references.
// 2. references must always be valid.

pub fn run() {
    let mut arg: String = std::env::args().nth(2).unwrap_or_else(|| {
        println!("Please supply second argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}

fn eat(s: String) -> bool {
    s.starts_with('b') && s.contains('a')
}

fn inspect(s: &String) {
    if s.ends_with('s') {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

fn change(s: &mut String) {
    if !s.ends_with('s') {
        s.push('s');
    }
}
