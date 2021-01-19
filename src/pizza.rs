enum Toppings {
    Tyre,
    
}

struct Pizza {
    name: String,
    cheese: bool,
    toppings: Option<Toppings>,
}

impl Pizza {
    
    pub fn new(name: &str, cheese: bool, toppings: Option<Toppings>) -> Self {
        Self {
            name: name.to_string(),
            cheese,
            toppings,
        }
    }

}