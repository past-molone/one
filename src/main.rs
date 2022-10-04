use std::io::{self, Write};

mod vecotr;

fn main() {

    print!("Please enter a number 1 or 2: ");

    io::stdout().flush().unwrap();

    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("");

    let a = a.trim();

    println!("----------------------------------------------------------------");

    if a.parse::<isize>() == Ok(1){
        for number in 1..11{
            println!("{number}");
        }
    }
    else if a.parse::<isize>() == Ok(2){
        for number in (1..11).rev(){
            println!("{number}");
        }
    }

    vecotr::pv();   

}