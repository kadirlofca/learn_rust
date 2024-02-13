use std::vec;

fn vectors() { // Store values of the same type.
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = vec![0, 0, 0];
    v3.push(0); // We can push elements to mutable vectors.

    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[100]; // This will crash.
    let does_not_exist = v.get(100); // This will return None.

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn strings() {
    // They're just like Vectors!
    let mut s = String::from("foo");
    s.push_str("bar"); // Will result in "foobar".

    s = s + "bar"; // Now it's "foobarbar".

    for c in "Зд".chars() { // Retrieving chars from a String.
        println!("{c}");
    }
}

fn hashmaps() {
    use std::collections::HashMap;

    // Making a hashtable.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Retrieving from a hashtable.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Looping a hashtable.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hashmap is the owner of the values inside it.

    // We can enter unique values into a hashmap with .entry().
    scores.entry(String::from("Blue")).or_insert(50); // .or_insert() returns a mutable reference to the value.

    // Iterating and updating values in hashtable.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}