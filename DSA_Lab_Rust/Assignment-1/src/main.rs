use std::io::Write;
use std::{env, fs::File, io::BufRead, io::BufReader};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 2 arguments, exiting!")
    }
    let reader = BufReader::new(File::open(&args[1]).expect("File not found!"));
    let mut num_of_digits: i32 = 0;
    let mut sum: i32 = 0;
    let mut min: i32 = 0;
    let mut max: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().parse::<i32>().unwrap();
        num_of_digits += 1;
        sum += line;
        if num_of_digits == 1 {
            min = line;
            max = line;
        } else {
            if line > max {
                max = line;
            }
            if line < min {
                min = line;
            }
        }
    }
    let average: f32 = sum as f32 / num_of_digits as f32;
    let mut f = File::create("output.txt").unwrap();
    f.write_all(format!("{}\n", num_of_digits).as_bytes())
        .expect("Issue");
    f.write_all(format!("{}\n", min).as_bytes())
        .expect("Issue 2");
    f.write_all(format!("{}\n", max).as_bytes())
        .expect("Issue 3");
    f.write_all(format!("{}\n", sum).as_bytes())
        .expect("Issue 4");
    f.write_all(format!("{:.2}\n", average).as_bytes())
        .expect("Issue 5");
}
