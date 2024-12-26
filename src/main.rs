fn get_name(name: &str) -> String {
    return "hello world".to_string() + name;
}

fn main() {
    println!("Hello, world!");

    print!("Enter your name: ");
    println!("{}", get_name("zhang san"));
}
