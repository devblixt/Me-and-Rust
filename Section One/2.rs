fn main() {
    let test = String::from("testString");
    for (i,c) in test.bytes().enumerate() {
        println!("So {} is {}",i,c);
    }
}