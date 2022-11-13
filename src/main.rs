use std::env;

fn main() {
    println!("{}", "hola");
    println!("{}", "hola");
    println!("{}", "hola");

    let args: Vec<_> = env::args().collect();
    
    println!("{:?}", args);
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }
}
