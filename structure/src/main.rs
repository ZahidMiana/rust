
//struct key value pair just like obj in java script and dictionary like py
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let mut user_1 = User{
        active: true,
        username: String::from("Zahid Iqbal"),
        email: String::from("zahidmiana56@gmail.com"),
        sign_in_count: 4,
    };

    println!("The name of User is {} ", user_1.username);

    user_1.username = String::from("Tahir Iqbal");
    user_1.username.push_str(" Miana");

     println!("The name of User is {} ", user_1.username);

     println!("======  Using FUnction we also create a struct lets see =====");
     let user_2: User = build_user(
        String::from("Iqra Younis"),
        String::from("admin@mygamil.com"),
     );

    println!("The name of User is {} ", user_2.username);


    println!("\n===========  Calculate the Area of Rectangle ==========\n");
    //this is for tuple now we see with struct
    // let area: (u32, u32) = (30, 40);
    // let result = calculate_area(area);

     let rect: Rectangle = Rectangle { 
        width: 10,
        height: 10, 
    };

    let result: u32 = calculate_area(&rect);

    println!("Area of {:?} is {}", rect, result);
}


fn build_user(username: String , email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0,
    }
}

fn calculate_area (rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
