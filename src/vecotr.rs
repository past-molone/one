pub fn pv(){
    let v1 = vec![1,2,3,4,5,6,7,8,9,10];

    for members in &v1{
        if members != &v1[8] && members != &v1[9]{
            print!("{:?}, ",members);
        }
        else if members == &v1[8]{
            print!("{members}");
        }
        else {
            println!("");
            print!("And the last member is : {members}");  
        }
    }
}
