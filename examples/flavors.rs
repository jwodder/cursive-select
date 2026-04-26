use cursivelect::{Curselect, Selector};
use mitsein::vec1::vec1;

fn main() {
    let mut app = Curselect::new();
    app.add(
        "flavor",
        Selector::Single {
            title: "Flavors:".to_owned(),
            options: vec1![
                "Vanilla".to_owned(),
                "Chocolate".to_owned(),
                "Strawberry".to_owned(),
                "Cinnamon".to_owned(),
                "Butterscotch".to_owned(),
                "Peanut Butter Fudge".to_owned(),
                "Chili".to_owned(),
            ],
            default: 0,
        },
    );
    app.add(
        "toppings",
        Selector::Multi {
            title: "Toppings:".to_owned(),
            options: vec1![
                "Whipped Cream".to_owned(),
                "Hot Fudge".to_owned(),
                "Nuts".to_owned(),
                "Cherry".to_owned(),
                "Banana".to_owned(),
            ],
        },
    );
    let selections = app.run();
    println!("{selections:#?}");
}
