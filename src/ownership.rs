pub fn ownership() {
    // Stack
    // All data must have a known, fixed size.
    // Pushing an element put's it at on top.
    // Popping removes the element at the top.
    // Last in, first out.

    // Heap
    // You request space from the heap, the memory allocator finds an empty spot and marks it as
    // being in use, then returns a pointer that addresses that location. This is called allocating.

    // Stack is faster than heap because the allocator never has to search for the place to store new data.

    // When a function is called, values passes into the function and local variables get pushed onto
    // the stack. When the function is over, those values get popped off the stack.


    // Ownership
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // Scope
    // s is not valid here because it is not declared yet.
    let s = "hello"; // s is valid from this point forward

    // String literals are immutable.
    // The String class manages data allocated and makes it possible for strings to be mutable.
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let x = 5;
    let y = x; // This makes pushes two copies of "5" onto the stack.

    let string_one = String::from("hello");
    let string_two = string_one; // This is a clone!! string_one and string_two point to the same place on the heap.
    // Rust considers string_one as invalid now!
    println!("{string_one}"); // This will cause an error because string_one is invalid.

    // If we want to deep copy (copy the value), we can use .clone();
    let string_three = string_two.clone();
    println!("{string_two}"); // Cloning string_two into string_three did not cause it to become invalid!

    // Passing ownership into functions.
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    // Retrieving ownership from functions.
    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    // Tuples can be passed and returned from functions.
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    // Taking ownership is nice, but what if we want to use a variable without passing ownership?
    // We can use &references!!
    calculate_length_reference(&s1); // We did not transfer ownership, just made a reference.

    // &References are immutable by default, but we can make them mutable.
    let mut s = String::from("hello");
    change(&mut s);
    // We can only borrow a mutable once per scope.

    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5]; // hello is a pointer to a slice from s.
                                    // &s[starting_index..ending_index (exclusive)]

    // Range note
    // To start a .. range from 0, do this:
    let slide = &s[..2]; // This range is equal to 0..2
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String

//    let s = String::from("hello"); // s is a new String

//    &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// Rules of References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.