fn main() {
    if std::env::args().len() == 1 {
        println!("You need to provide the file lol");
        std::process::exit(0);
    }

    let mut file: Vec<String> = std::env::args().collect();
    file.remove(0);
    let chosen = &file[0];

    let contents: String = std::fs::read_to_string(chosen).expect("Error!");
    let split = contents.split("\n");

    let mut len: usize = 0;

    for _ in split {
        len = len + 1;
    }

    println!("{}", &len)
}
