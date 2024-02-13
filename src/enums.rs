enum IpAddress {
    V6(String), // Enums can optionally store values!!!!!!!!
    V4(u8, u8, u8, u8) // Tuples also work!
}

// Enums also support impl.
impl IpAddress {
    fn all(&self) {
    }
}

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn enums() {
    let home = IpAddress::V6(String::from("127.0.0.1"));

}

fn option_and_match() {
    // Option is Rusts version of null references.
    // Option is an enum and has two values: None, and Some<T>.
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // You need to extract T from Option<T> to access it.
    // You cannot do, for example, Option<i32> + i32, because they are different types.
    // We can use the match expression to handle Option.

    let coin_value = Coin::Dime;
    match coin_value {
        Coin::Penny => 1,
        Coin::Nickel => 5, // These are called arms.
        Coin::Dime => {
            // For multiple lines, we can open up curly brackets.
            10
        },
        Coin::Quarter(state) => { // We can handle enum values like this.
            println!("{state}");
            25
        }
    };

    // Let's increment a potentially null number.
    let number = Some(1);
    match number {
        None => println!("number is null D:"),
        Some(n) => Some(n + 1)
    }

    // Matches are exhaustive.
    // You must handle all possible arms.

    // We can use the catch-all arm to handle multiple patters at once.
    // Catch all arm comes last.
    let dice_roll = 9;
    match dice_roll {
        6 => println!("Rolled 6!"),
        other => println!("Rolled {other}!") // This is the catch-all arm.
    }

    // If we don't need the variable ("other" above), we can use the _ pattern.as
    match dice_roll {
        6 => println!("Rolled 6!"),
        _ => println!("Not 6 D:")
    }

    // If nothing should happen, use ().
    match dice_roll {
        6 => println!("Rolled 6!"),
        _ => ()
    }
}

fn if_let() {
    // Match is kinda annoying if we're only executing code for one pattern.
    // We can use if let instead!

    let config_max = Some(3u8); // 3 of type u8
    if let Some(max) = config_max {
        println!("The maximum is {}", max);
    }

    // We can use else with if let.
    if let None = config_max {
        println!("Invalid maximum D:")
    }
    else {
        println!("The maximum is valid!");
    }
}