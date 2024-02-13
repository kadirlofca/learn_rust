fn generics() {
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    struct Point<T> {
        x: T,
        y: T // Structs can also have generics!
    }

    impl<T> Point<T> {
        fn x(&self) -> &T { // We can make methods generic as well!
            &self.x
        }
    }

    impl Point<i32> { // We can define extra implementations specific to types.

    }


    // Note that x and y are the same type, you can't pass in different types!
// If we need multiple generics, we can pass in multiple generics.
    struct Point2<T, U> {
        x: T,
        y: U,
    }

// Rust compiler applies monomorphization. It reads the types that are assigned to the generic and converts them into
// non-generic functions to be used.
// The monomorphized function names look like "Option_i32, Option_f64, etc".
}

fn traits() {
    // Traits define functionality of a type that can be shared with other types.

    pub trait Summary {
        fn summarize(&self) -> String;

        fn summarize2(&self) -> String {
            String::from("Read More...") // We can implement default behavior.
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("myusername"),
        content: String::from("Conteeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeent!"),
    };

    println!("1 new tweet: {}", tweet.summarize());
}