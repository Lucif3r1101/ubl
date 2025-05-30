pub fn run(input: &str, output: &str, password: Option<String>) {
    println!("Compressing '{}' into '{}'", input, output);
    if let Some(pwd) = password {
        println!("Using password: {}", pwd);
    }
}
