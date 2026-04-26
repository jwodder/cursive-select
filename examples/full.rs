use cursive_select::{Form, RadioSelector, Selection};

const WORDS: [&str; 26] = [
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
];

const NUMBERS: [&str; 6] = ["Zero", "One", "Two", "Three", "Four", "Five"];

fn main() {
    let mut app = Form::new();
    app.add("word", RadioSelector::new("Code Word:", WORDS));
    app.add(
        "number",
        RadioSelector::new("Code Number:", NUMBERS).with_default(5),
    );
    let selections = app.run();
    if std::env::args_os().nth(1).is_some_and(|s| s == "--debug") {
        println!("{selections:#?}");
    } else if let Some(selections) = selections {
        for (k, v) in selections {
            match (k, v) {
                ("word", Selection::Radio(i)) if i < WORDS.len() => {
                    println!("Code Word: {} ({i})", WORDS[i]);
                }
                ("word", Selection::Radio(i)) => println!("Code Word: #{i}"),
                ("number", Selection::Radio(i)) if i < NUMBERS.len() => {
                    println!("Code Number: {} ({i})", NUMBERS[i]);
                }
                ("number", Selection::Radio(i)) => println!("Code Number: #{i}"),
                (k, v) => println!("UNEXPECTED OUTPUT: ({k:?}, {v:?})"),
            }
        }
    } else {
        println!("User quit");
    }
}
