use cursivelect::{Curselect, MultiSelector, RadioSelector, Selection};

const FLAVORS: [&str; 7] = [
    "Vanilla",
    "Chocolate",
    "Strawberry",
    "Cinnamon",
    "Butterscotch",
    "Peanut Butter Fudge",
    "Chili",
];

const TOPPINGS: [&str; 5] = ["Whipped Cream", "Hot Fudge", "Nuts", "Cherry", "Banana"];

fn main() {
    let mut app = Curselect::new();
    app.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    app.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let selections = app.run();
    if std::env::args_os().nth(1).is_some_and(|s| s == "--debug") {
        println!("{selections:#?}");
    } else if let Some(selections) = selections {
        for (k, v) in selections {
            match (k, v) {
                ("flavor", Selection::Radio(i)) if i < FLAVORS.len() => {
                    println!("Flavor: {} ({i})", FLAVORS[i]);
                }
                ("flavor", Selection::Radio(i)) => println!("Flavor: #{i}"),
                ("toppings", Selection::Multi(tops)) if tops.is_empty() => {
                    println!("Toppings: -- none --");
                }
                ("toppings", Selection::Multi(tops)) => {
                    println!("Toppings:");
                    for t in tops {
                        if t < TOPPINGS.len() {
                            println!(" - {} ({t})", TOPPINGS[t]);
                        } else {
                            println!(" - #{t}");
                        }
                    }
                }
                (k, v) => println!("UNEXPECTED OUTPUT: ({k:?}, {v:?})"),
            }
        }
    } else {
        println!("User quit");
    }
}
