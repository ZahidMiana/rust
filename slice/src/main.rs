fn main() {
    let s: String = String::from("Hello World");

    let result = find_first_word(s);
    println!("Result is: {result}");
}

fn find_first_word(input: String) -> usize {
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    input.len()
}


