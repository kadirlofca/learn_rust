struct User {
    active: bool,
    username: String,
    // username: &str, // This would not compile because it requires a "lifetime" to handle ownership.
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // Structs can also be written like tuples!

struct AlwaysEqual; // Structs can also be written like a unit ().
                    // This is called a unit-struct.

#[derive(Debug)] // This macro will help us log this struct for debugging.
struct Rectangle {
    width: u32,
    height: u32
}

// We can use methods to add logic to our structs.
// impl stands for implementation.
impl Rectangle {
    fn area_method(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // Multiple impl blocks are possible.
}

fn using_structs() {
    let black = Color(0, 0, 0);
    black.0; // You can access using .index.

    let subject = AlwaysEqual;

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 123,
    };

    let mut mutable_user = User  {
        active: true,
        username: String::from("someusername456"),
        email: String::from("someone_else@example.com"),
        sign_in_count: 123,
    };

    // Access struct values using dot.
    // user1.email = String::from(""); // Assigning gives error because user1 is not mutable.
    mutable_user.email = String::from("newemail@example.com"); // This doesn't give error because it's mutable.

    // Transferring values from one struct to a new one.
    let transferred_user = User {
        email: String::from("newemail@example.com"),
        ..user1 // Set rest of the values to be the same as user1.
    };

    // user1 // Ownership has changed when we transferred values from user1 to transferred_user.
    // Now trying to use user1 causes an error.
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // Notice here we are using the convenience feature "field init" to initialize the struct value.
                  // Value name and parameter name needs to be the same.
        email,
        sign_in_count: 1,
    }
}

fn area (dimensions: (u32, u32)) -> u32 { // We can pass tuples like this to make it more readable!
    // This is better than having two parameters (width and height) because it holds them in a single tuple.
    dimensions.0 * dimensions.1

    // But what if, say, we wanted to render this rectangle on the screen? Using tuples, we can't know
    // which value is width and which is height. So using tuples is not always the best option.
}

// OR
// We can use structs to name values to remove confusion.

fn area_two (rectangle: &Rectangle) -> u32 {
    println!("{:?}", rectangle);
    // Or
    dbg!(rectangle);
    rectangle.width * rectangle.height;

    // We can also use:
    rectangle.area_method()
}