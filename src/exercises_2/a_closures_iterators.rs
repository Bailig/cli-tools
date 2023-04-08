// for loop automatically converts the collection into an iterator (e.g. vec.into_iter())
// into_iter() takes the ownership and returns an iterator
// that means once a collection is iterated, it is gone

// interator adaptors are methods that take an iterator as input and return a new iterator
// they are lazy, meaning they don't do anything until they are consumed by a consuming adaptor (e.g. for_each, collect, etc.)

// iterator consumers sometimes cannot figure out the type
// in that case, we need to explicitly specify the type using the turbofish syntax (::<>) or type annotation

// v.into_iter()   for _ in v         (consumes the collection)
// v.iter()        for _ in &v        (borrow the collection)
// v.iter_mut()    for _ in &mut v    (mutably borrow the collection)

pub fn run() {
    let square = |x| x * x;
    println!("5 squared is {}", square(5));

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    pairs
        .into_iter()
        .map(|(a, b)| (a + 1, b))
        .for_each(|t| println!("{:?}", t));

    let mut numbers = vec![1, 2, 3, 4];
    for x in numbers.iter_mut() {
        *x *= 3;
    }
    println!("{:?}", numbers);

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed: Vec<String> = words
        .into_iter()
        .filter(|&w| w.contains('h'))
        .map(|w| w.to_uppercase())
        .collect();
    println!("Transformed: {:?}", transformed);

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    for (a, b) in pairs {
        println!("{:?}", (a + 1, b));
    }

    let mut numbers = vec![1, 2, 3, 4];
    numbers.iter_mut().for_each(|n| *n *= 3);
    println!("{:?}", numbers);
}
