#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn create(width: u32,height: u32)-> Self {
        Self{
            width,
            height,
        }
    }
}

fn main(){
    let rect1 = Rectangle{
        width : 2,
        height : 2,
    };
    dbg!(&rect1);
    println!("The area is {}",rect1.area());
    let rect1 = Rectangle::create(2,3);
    dbg!(&rect1);
}

