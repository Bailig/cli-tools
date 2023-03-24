// str is a string slice.
// String is a string type.
// the differences are:
// 1. String is allocated on the heap, str is allocated on the stack.
// 2. String is mutable, str is immutable.
// 3. String is owned, str is borrowed.
// 4. String is a pointer to a buffer, str is a pointer to a string.

// a borrowed string slice is a pointer to some bytes and a length of the str.
// a String is a pointer to some bytes, a length, and a capacity. The capacity is higher than the length.
// both String and str are UTF-8 encoded.
// grapheme clusters are the smallest unit of a string that a human can perceive as a single character. (e.g. 你)
// a grapheme cluster is made up of one or more unicode scalar values. (e.g. 你 is made up of 2 unicode scalar values)
// a unicode scalar is made up of bytes.

// you can use .bytes() to get an iterator over the bytes of a string.
// use .as_bytes() and than index into the bytes to get a byte, and than cast it to a char. (e.g. "abc".as_bytes()[0] as char)
// you can use .chars() to get an iterator over the unicode scalar values of a string. (e.g. "abc".chars().nth(0).unwrap())

pub fn run() {
    let args: Vec<String> = std::env::args().skip(2).collect();
    if args.len() == 0 {
        println!("Please supply second argument to this program.");
        std::process::exit(-1);
    }

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;
    for i in 7..=23 {
        sum = sum + i;
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x <= 500 {
        x *= 2;
        count += 1;
    }
    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut i = 0;
    loop {
        if i > 7 {
            break;
        }
        print!("{} ", arg);
        i += 1;
    }
    println!();
}
