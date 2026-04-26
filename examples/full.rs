use cursivelect::{Curselect, RadioSelector};

fn main() {
    let mut app = Curselect::new();
    app.add(
        "word",
        RadioSelector::new(
            "Code Word:",
            [
                "Abacus",
                "Banana",
                "Coconut",
                "Delta",
                "Exotic",
                "Finagle",
                "Geranium",
                "Heliopause",
                "Indigo",
                "Justice",
                "Kangaroo",
                "Lemon",
                "Mausoleum",
                "Nocturnal",
                "Occupation",
                "Philosophy",
                "Quux",
                "Radius",
                "Service",
                "Tuxedo",
                "Universe",
                "Vulpine",
                "Wolpertinger",
                "Xylem",
                "Yellow",
                "Zyzzyva",
            ],
        ),
    );
    app.add(
        "number",
        RadioSelector::new(
            "Code Number:",
            [
                "Zero (0)",
                "One (1)",
                "Two (2)",
                "Three (3)",
                "Four (4)",
                "Five (5)",
            ],
        )
        .with_default(5),
    );
    let selections = app.run();
    println!("{selections:#?}");
}
