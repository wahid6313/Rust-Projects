fn main() {
    let mut x: i8 = 5;  
    
    for i in 0..1000 {
        x = x + 100;
    }
    
    print!("x: {}", x);

}

