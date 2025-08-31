fn main(){
    let answer = greeting();
    test_welcome();
    println!("String is {} ", answer);
}

fn greeting() -> & 'static str{
    "Hello My Name is Zahid Iqbal"
}

fn test_welcome() -> () {
    assert_eq!(greeting(), "Hello My Name is Zahid Iqbal");
}