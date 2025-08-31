#[derive(Debug)]
enum UsState{
    Albama,
    Alaska,
    NewYork
}

enum Coin {
    Penny,
    Nickle,
    Dime, 
    Quarter(UsState),
}

fn main() {
    let coin: Coin = Coin::Penny;
    println!("Value is {} ", value_in_cents(coin));

    println!("Add Result = {}", add(50, None));
}

fn add(num: i32, Option<i32>) -> Option<i32>{
    match num2 {
        Some(i: i32) => num +i,
        None => 0,
    }
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state: UsState) => {
            println!("Got Quarter of Value {:?}", state);
            25
        },
    }
}
