use std::process::Command;
use std::io::{self, Write};

fn main() {
    print!("enter file name");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let input = input.trim();
     let output = Command::new("ls")
    
         .arg(input)
       
         .output()
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
