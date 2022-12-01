fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let _third : Option<&i32> = v.get(100);
    // println!("Third test {:?}",third);
    // println!("Test {:?}",v);
    for i in &mut v {
        if *i%2!=0 {
        *i += 50;}
    }
    println!("Test {:?}",v);
}