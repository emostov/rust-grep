use std::env; 

fn main() {
    // Gather and creat vector cl args pass to program, will not accept invlid unicode
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}