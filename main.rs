fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    println!("You said: {:?}!", &args)
}
