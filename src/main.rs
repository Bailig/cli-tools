mod algorithms;
mod exercises;
mod exercises_2;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!(
            "Please supply first argument as either 'algorithms', 'exercises', or 'exercises_2'."
        );
        std::process::exit(1);
    }
    let arg = &args[0];
    if arg == "algorithms" {
        algorithms::array::run();
        algorithms::linked_list::run();
    } else if arg == "exercises" {
        println!("\nvariables:");
        exercises::a_variables::run();

        println!("\nfunctions:");
        exercises::b_functions::run();

        println!("\nsimple types:");
        exercises::c_simple_types::run();

        println!("\ncontrol flow and strings:");
        exercises::d_control_flow_strings::run();

        println!("\nownership and references:");
        exercises::e_ownership_references::run();

        println!("\nstruct and traits:");
        exercises::f_structs_traits::run();

        println!("\ncollections and enums:");
        exercises::g_collections_enums::run();

        println!("\n closures and threads:");
        exercises::h_closures_threads::run();
    } else if arg == "exercises_2" {
        println!("\n closures and iterators:");
        exercises_2::a_closures_iterators::run();

        println!("\n traits:");
        exercises_2::b_traits::run();
    } else {
        println!("Unknown argument: {}", arg);
        std::process::exit(1);
    }
}
