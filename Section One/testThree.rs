fn main(){
    let s1 = String::from("s1");
    let s2 = "string literal";
    let s3 = String::from("s3");
    takes_ownership(s2);
    println!("test 1 {}",s2);//works, since s2 has Copy trait implemented
    takes_ownership_string(s1);
    println!("test 1 {}",s1);//does not work since s1 is already out of scope
 
}

fn takes_ownership( s: &str){
    //do nothing
}

fn takes_ownership_string( s: String){
    //do nothing
}

//string literal must be on stack, so it does not go out scope after takes_ownership gets s2

