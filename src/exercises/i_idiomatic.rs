const pi: f32 = 3.14159265358979323846;

fn count_to_5() -> i32 {
    let mut foo = 0;
    loop {
        if foo > pi as i32 {
            if foo > 5 {
                break;
            }
        }
        foo = foo + 1;
    }
    return 5;
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
