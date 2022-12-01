#[derive(Debug)]
enum TypeStud {
    Piro(Dept),
    Nub(Dept),
}
#[derive(Debug)]
enum Dept {
    CSE,
    EE,
    ME,
}

fn main(){
    let prabhu = TypeStud::Piro(Dept::CSE);
    match prabhu {
        TypeStud::Piro(dept) => {
            println!("Prabhu is Piro and from department {:#?}",dept);
        },
        TypeStud::Nub(dept)=>{},
    }
}