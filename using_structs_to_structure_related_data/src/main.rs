struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

struct ColorRGB(u8, u8, u8);
struct Point2D(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let user = User {
        email: String::from("hiragi-gkuth@hiragi.guru"),
        username: String::from("hiragi-gkuth"),
        active: true,
        sign_in_count: 1,
    };

    let red = ColorRGB(255, 0, 0);

    let subject = AlwaysEqual;

    // some fields 'move' to user2, other fields have 'copy' trait
    let user2 = User {
        active: false,
        ..user
    };

    // not a problem! because of bool that is primitive is 'copy' trait.
    println!("{}", user.active);
    // an error occured because of String assignment is 'move'
    // println!("{}", user.email);
}
