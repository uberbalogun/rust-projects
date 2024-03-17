enum Colour{

    Blue,
    Green,
    Orange,
    White,
}

fn show_colour(my_colour: Colour) {

    match my_colour {
        Colour::Blue => println!("It's blue!"),
        Colour::Green => println!("It's green!"),
        Colour::Orange => println!("It's orange!"),
        Colour::White => println!("It's white!"),
    }

}

fn main() {

    show_colour(Colour::Blue);

}
