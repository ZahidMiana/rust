fn main() {
    //Scalar Datatypes
    //1)integer 2) float 3) char 4) bool
    //integer
    let a: u8 = random_number() + 100;
    println!("Value is {a}");

    //compund datatypes
    //1) Tuples  2) Arrays
    let tup: (u32, f64, bool) = (123, 6.8, true);
    let a: [i8; 7] = [1,2,3,4,5,6,7];
}

fn random_number() -> u8 {
    200
}
