enum Toppings {
    Pepperoni,
    Sardines,
    Onions,
    Bacon
}

impl Toppings {
    fn to_string(self) -> String {
        match self {
            Toppings::Pepperoni => String::from("Tire"),
            Toppings::Sardines => String::from("Tire"),
            Toppings::Onions => String::from("Tire"),
            Toppings::Bacon => String::from("Tire"),
        }
    }
}

pub fn available_toppings() {
    let t1: Toppings = Toppings::Pepperoni;
    let t2: Toppings = Toppings::Sardines;
    let t3: Toppings = Toppings::Onions;
    let t4: Toppings = Toppings::Bacon;


    println!("Available toppings:");
    println!("{}", t1.to_string());
    println!("{}", t2.to_string());
    println!("{}", t3.to_string());
    println!("{}", t4.to_string());

}