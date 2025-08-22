fn main() {

    //variables
    let mut age: i32 = 23;
    println!("Value of age is {age}");

    age =25;
    println!("Value of age is {age}");

    //constant
    const PI: f64 = 3.1416; //it throw error because const ko type denna must hai
    println!("PI Value is {PI}");

    //shadowing
    let apple: i32 = 23;
    println!("Value of Apple {apple}");

    let apple: bool = true;
    println!("Value of Apple {apple}");
}