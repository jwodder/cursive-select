use cursivelect::{Curselect, MultiSelector, RadioSelector};

fn main() {
    let mut app = Curselect::new();
    app.add(
        "flavor",
        RadioSelector::new(
            "Flavors:",
            [
                "Vanilla",
                "Chocolate",
                "Strawberry",
                "Cinnamon",
                "Butterscotch",
                "Peanut Butter Fudge",
                "Chili",
            ],
        ),
    );
    app.add(
        "toppings",
        MultiSelector::new(
            "Toppings:",
            ["Whipped Cream", "Hot Fudge", "Nuts", "Cherry", "Banana"],
        ),
    );
    let selections = app.run();
    println!("{selections:#?}");
}
