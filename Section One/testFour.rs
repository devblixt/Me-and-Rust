fn main(){
    let s1 = String::from("First Second");
    let s2 = first_word(&s1);
    println!("The result : {}",s2);
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            println!("The number is {}",i);
            return &s[..i];
        }
    }
    &s[..]
}