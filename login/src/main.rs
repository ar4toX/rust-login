use std::io;

fn main() {
    let count:i32=0;
    enter_password(count);
}

fn enter_password(mut count: i32) {
    let mut password = String::new();

    println!("Enter the password: ");
    io::stdin().read_line(&mut password)
        .expect("Failed to read password");

    let password = password.trim().parse::<i32>()
        .expect("Invalid password");

    if password == 1234 {
        println!("Access granted");
    } else {
        if count<=4 {
            println!("Access denied");
            count+=1;
            enter_password(count);
        }
        else {
            println!("Too many attempts, die!");
        }
    }
}