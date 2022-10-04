pub fn pv(){
    let mut v1 = vec!(1..11);
    v1.push(11..13);

    for members in v1{
        print!("{:?}",members);
    }
}
