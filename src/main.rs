// An example of structs in Rust

// Defining a struct:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Defining tuple structs:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Defining a unit-like struct
struct AlwaysEqual;

fn main() {
    // Creating an instance of our struct:
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Accessing struct members:
    // user1.active;
    // user1.username;
    // user1.email;
    // user1.sign_in_count;

    // Creating a mutable struct using our function.
    let mut user2 = build_user("someuser@example.com", "someuser");

    // Only allowed if the entire struct is mutable:
    user2.email = String::from("anotheremail@example.com");

    // Creates a new user WITHOUT using struct update syntax.
    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Creates a new user WITH struct update syntax.
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user2 // This means copy everything else from user2
    };

    // Note that, due to moving user1 and user2 features into user3, ownership has passed to user3.
    // The following lines will give an error, since they'd be references to moved variables:
    // user1.username;
    // user2.username;

    // Examples of creating tuple struct instances. Note that the elements are accessed through indexing, since they don't have names.
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    // let invalid: Color = Point(1, 2, 3); // Error! Even though they have the same type of elements, they're still seperate structs!

    // Creating an instance of a unit-like struct. Useful for implementing traits, as discussed in Ch 10.
    let _subject = AlwaysEqual;
}

// Building a struct as a function
fn build_user(mail: &str, uname: &str) -> User {
    User {
        active: true,
        username: String::from(uname),
        email: String::from(mail),
        sign_in_count: 0,
    }
}

// Uses field init shorthand to create a user. This means we don't have to repetitively assign values if the input has the same name.
fn _build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}