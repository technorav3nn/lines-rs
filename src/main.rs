use std::{env, fs, process};

fn main() {
    if env::args().len() == 1 {
        println!("You need to provide the file lol");
        process::exit(0);
    }

    let mut file: Vec<String> = env::args().collect();
    file.remove(0);
    let contents: String = fs::read_to_string(&file[0]).expect("Error!");
    let split = contents.split("\n");
    let len = split.collect::<Vec<&str>>().len();
    println!("{}", &len)
}
