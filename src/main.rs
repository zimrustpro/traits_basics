use std::fmt::Display;

struct Cat {
    name: String,
    age: u8,
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!("{mr_mantle}");
    print_excitedly(mr_mantle.to_string());
    println!(
        "Mr. Mantle's String is {} characters long.",
        mr_mantle.to_string().chars().count()
    );
}

fn print_excitedly(input: String) {
    println!("{input}!!!!!");
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}
