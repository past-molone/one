pub fn pv(){
    let v1 = vec!(1;10);

    for members in v1{
        if members != 2{
            print!("{:?}, ",members);
        }
        else {
            print!("This is the last member: {members}");  
        }
    }

    let x = 5;
    let y = x;
    print!("{x},{y}")
}
