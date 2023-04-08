// struct allows to define a new data type but not methods
// trait allows to define new methods
// impl allows to implement methods (traits) for a struct

pub fn run() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(&mut self) {
        self.percent_left *= 0.8;
    }
}

trait Bite {
    fn bite(&mut self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.amount_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(biteable: &mut T) {
    biteable.bite();
}
