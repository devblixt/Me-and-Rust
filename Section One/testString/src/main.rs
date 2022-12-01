fn main() {
    let s1 = String::from("helo world");
    let s2=s1;
    println!("yeah {}",s1);
}
//works, doesnt print because s2 borrows s1 from heap (should not free twice)