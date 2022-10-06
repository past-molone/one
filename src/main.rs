use std::io::{self, Write};

mod vector;
fn main() {

    println!("Enter 1 for ascending numerical order or 2 for vice versa.");
    print!("Please enter a number 1 or 2: ");

    io::stdout().flush().unwrap();

    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("");

    let a = a.trim();

    println!("----------------------------------------------------------------");

    if a.parse::<i32>() == Ok(1){
        for number in 1..11{
            println!("{number}");
        }
    }
    else if a.parse::<i32>() == Ok(2){
        for number in (1..11).rev(){
            println!("{number}");
        }
    }
    else{
        println!("Ey bro, you don't understand what I'm saying huh.");
    }

    vector::pv();   

}