use std::io::{self, Write};

mod sidekick;
fn main() {

    println!("Enter 1 for ascending numerical order or 2 for the opposite.");
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
        println!("Ey bro, do you understand what I'm saying huh.");
    }

    sidekick::pv();   

    // let mut b = String::new();
    // io::stdin()
    //     .read_line(&mut b)
    //     .expect("");

    // sidekick::ps(&mut b);

}