&String for normal reference
&mut String for mutable reference
you cant use multiple mutable references to the same variable at the same time

if immutable reference is created, a mutable reference to same variable cannot be created before immutable reference is used : 

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}


what if a reference is passed on to variable (let a = &b) and the data to which reference is pointing to, expires?
rust prevents this, so that dangling pointers(pointers pointing to invalid/inaccessble memory locations) are not possible