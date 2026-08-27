use std::env;

fn greet(name: &str) {
    println!("Hello World! {}", name);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        greet(&args[1]);
    } else {
        println!("Please, provide name");
    }
}
