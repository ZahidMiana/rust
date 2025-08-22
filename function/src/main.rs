fn main() {
    println!("Hello, world!");
    my_function();
    parameter_function(7, true);

    let result = addition(10, 20);
    println!("The rsult is {result}");
}

fn my_function(){
    println!("Hello I am from my Function");
}

fn parameter_function(x: i32, y: bool){
    println!("Hello from parameter function {x} {y}");
}

fn addition(x: i32, y: i32) -> i32{
     x+y
}
