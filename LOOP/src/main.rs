fn main() {

    println!("======For LOop=====");
    let mut x:i32 = 1;

    loop {
        println!("Value is: {x}");

        if x==10 {
            break;
        }
        x = x+1;
    }

    println!("======WHile LOop=======");
    let mut y: i32 = 5;

    while y!= 0 {

        println!("Value is: {y}");
        y= y-1;

    }
    
}
