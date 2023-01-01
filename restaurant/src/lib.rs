mod back_of_house;
mod front_of_house;

pub use front_of_house::hosting;
pub use back_of_house::Appetizer;

pub fn eat_at_rastaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
