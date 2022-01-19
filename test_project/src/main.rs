fn main() {
    println!("This is a test project to get used to working with cargo.");
    println!("Oh and also getting reacquainted with GitHub since I use TFVC for my 9-5...");
    println!("To terminate press [ANY] key...");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("This shouldn't happen but we were terminating anyway so no biggie :)");
}
