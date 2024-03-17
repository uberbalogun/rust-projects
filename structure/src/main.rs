enum Flavor {
    Sweet,
    Sour,
    Pungent,
}
struct Drink {
    flavor: Flavor,
    fl_oz: f64,
}

fn get_flavor(drink: Drink) {

    match drink.flavor {
        Flavor::Sweet => println!("this is sweet!"),
        Flavor::Sour => println!("this is sour!"),
        Flavor::Pungent => println!("this is pungent!"),
    }

    println!("size: {:?}", drink.fl_oz);
}
fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fl_oz: 7.0
    };

    get_flavor(sweet)
}
