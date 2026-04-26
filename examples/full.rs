use cursivelect::{Curselect, Selector};
use mitsein::vec1::vec1;

fn main() {
    let mut app = Curselect::new();
    app.add(
        "word",
        Selector::Single {
            title: "Code Word:".into(),
            options: vec1![
                "Abacus".into(),
                "Banana".into(),
                "Coconut".into()
                "Delta".into(),
                "Exotic".into(),
                "Finagle".into(),
                "Geranium".into(),
                "Heliopause".into(),
                "Indigo".into(),
                "Justice".into(),
                "Kangaroo".into(),
                "Lemon".into(),
                "Mausoleum".into(),
                "Nocturnal".into(),
                "Occupation".into(),
                "Philosophy".into(),
                "Quux".into(),
                "Radius".into(),
                "Service".into(),
                "Tuxedo".into(),
                "Universe".into(),
                "Vulpine".into(),
                "Wolpertinger".into(),
                "Xylem".into(),
                "Yellow".into(),
                "Zyzzyva".into(),
            ],
            default: 0,
        },
    );
    app.add(
        "number",
        Selector::Single {
            title: "Code Number:".into(),
            options: vec1![
                "Zero (0)".into(),
                "One (1)".into(),
                "Two (2)".into(),
                "Three (3)".into(),
                "Four (4)".into(),
                "Five (5)".into(),
            ],
            default: 5,
        },
    );
    let selections = app.run();
    println!("{selections:#?}");
}
