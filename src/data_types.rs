pub fn mutability() {
    let x = 1;
    println!("{x}");
    // x = 2; causes error.

    let mut y = 5;
    println!("{y}");
    y = 10; // Does not cause error because y is mutable.
    println!("{y}");
}

pub fn tuples() {
    // Tuples are collections of multiple types of elements.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let x = tup.0; // Access tuple elements using tuple.element_index.
    println!("{x}");

    let tup_inferred = (600, 7.5, 2);
    let (x, y, z) = tup_inferred;
    println!("{y}");
}

pub fn arrays() {
    // Arrays are fixed size unlike vectors.
    let months = ["January", "February", "March", "April",
                            "May", "June", "July", "August", "September",
                            "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Notation is array: [type; length] = [elements].

    let b = [0, 5]; // Initializes array with five zeroes.
    let b_first_element = b[0];
}

pub fn statements_and_expressions() { // Function definitions are statements.
    let x = 5; // This is a statement.

    // Statements do not return values.
    // let x = (let y = 6) // This will give an error.

    // A math operation is an expression.
    // Calling a function is an expression.
    // Calling a macro is an expression.
    // A new scope block created with curly brackets is an expression.

    let y = {
        let x = 3; // This is a statement.
        x + 1 // This math operation expression returns a value, assigning it to y.
              // See how there is no semicolon after "x+1". This is because it is an expression.
              // Expressions don't end with semicolons.
              // Semicolons are for statements.
    }; // The brackets with the stuff inside is an expression.

    println!("{y}");
}

pub fn functions_and_return_values() -> i32 { // Return type notation is function() -> type.
    5 // This function returns 5.
    // or
    // return 5;
}

pub fn control_flow(number: i32) { // Same old (without parenthesis).
    if number < 5 {
        println!("Number is smaller than 5!");
    }
    else if number == 5 { // Rust only runs the first true arm. Does not even check if the rest are true or false.
        println!("Number is equal to 5!");
    }
    else {
        println!("Number is greater than 5!");
    }

    // If statements are expressions, so we can use them to assign values to variables.
    let is_greater: &str = if number > 5 {"yes"} else {"no"};

    // You can't mix return types.
    // let is_greater = if number > 5 {5} else {"yes"}; // This will cause error.

    loop {
        println!("again!");
        break; // Break out of loop.
        continue; // Skip rest of the code and restart loop.
    }

    let mut counter = 0;
    let result = loop { // Loops can return values!!!
        counter += 1;

        if counter == 10 {
            break counter * 2; // This is how you return a value from loop.
        }
    };

    println!("Result is {result}");


    // Labeling loops.
    'parent_loop: loop { // This is how you label a loop.
        loop {
            //break; // This will break from inner loop.
            break 'parent_loop; // This will break from outer loop.
        }
    }

    let mut while_condition = true;
    while while_condition {
        while_condition = false;
    }

    let array = [1, 2, 3, 4, 5];
    for element in array {
        println!("{element}");
    }

    // Countdown using a loop.
    println!("asdas");
    for number in (1..5).rev() { // .rev() reverses the range.
        // (5..1) doesn't work.
        println!("{number}");
    }
}