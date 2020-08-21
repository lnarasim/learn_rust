#[derive(Debug)]
struct SomeThing;

impl Drop for SomeThing {
    fn drop(&mut self) {
        println!("Dropping {:?}", self)
    }
}

fn main() {
    let y: SomeThing; 
    {
        let x = SomeThing;
        println!("printing x {:?}" , x);
        y = x;
    }
    println!("printing y {:?}", y);
}
