fn main() {

    let mut s = String::from("Zahid Iqbal");
    s.push_str(", Miana");
    
    println!("{s}");



    let s1 = String::from("Hello");
    let mut s2 = s1.clone();

    s2.push_str(", World");
    println!("S1= {s1}, S2= {s2}");


    let x = 10;
    let y = x;

    println!("x = {x}  y= {y}");


    let str=String::from("O ki haal hai");
    take_ownership(str);

    let h =10;
    make_copy(h);

    let owner = give_ownership();
    println!("{owner}");

    let maser = String::from("JI JI Jessy Ap kahein");
    let agla =take_and_giveback(maser);
    println!("{agla}");


    let name = String::from("Babr Azam");

    let (naa, ln) = calculate_length(name);
    println!("The length of {naa} is {ln}");



}

fn  take_ownership(some_string: String){
    println!("{some_string}");
}

fn make_copy(some_integer: i32){
    println!("{some_integer}");
}

fn give_ownership() -> String {
    let some_string = String::from("Hn Nw aessa hi hai");
    some_string
}


fn take_and_giveback(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}