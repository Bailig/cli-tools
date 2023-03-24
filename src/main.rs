mod algorithms;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Missing argument: algorithms or exercises");
        std::process::exit(1);
    }
    let arg = &args[0];
    if arg == "algorithms" {
        algorithms::array::run();
        algorithms::linked_list::run();
    } else if arg == "exercises" {
    } else {
        println!("Unknown argument: {}", arg);
        std::process::exit(1);
    }
}
