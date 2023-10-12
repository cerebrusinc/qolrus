use qolrus::{random_colour, ColourType};

fn main() {
    let colour: String = random_colour(ColourType::HEX);
    println!("The colour is: '{}'", colour)
}
