fn main() {
    use std::collections::HashMap;
    use std::io;
    let mut map: HashMap<&str,Vec<&str>> = HashMap::new();
    let tech = vec!["Hari","Devdatt"];
    let sales = vec!["Superman","Spiderman"];
    map.insert("tech",tech);
    map.insert("sales",sales);
    let stdin = io::stdin();
    println!("Enter input");
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).expect("Failed to read line");
    let user_input = user_input.trim().to_string();
    let vec = map.get_mut("tech");
    vec.expect("No idea").push(&user_input);
    println!("{:?}",map);
}