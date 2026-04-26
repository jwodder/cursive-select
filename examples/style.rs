use cursive_select::{Form, MultiSelector, RadioSelector, Selection};

const EFFECTS: [&str; 9] = [
    "Bold",
    "Dim",
    "Italic",
    "Underline",
    "Blink",
    "Fast Blink",
    "Reverse",
    "Hidden",
    "Strikethrough",
];

const COLORS: [&str; 8] = [
    "Black", "Red", "Green", "Yellow", "Blue", "Magneta", "Cyan", "White",
];

fn main() {
    let mut app = Form::new();
    app.add(
        "effects",
        MultiSelector::new("Effects:", EFFECTS).with_defaults([3, 7]),
    );
    app.add(
        "color",
        RadioSelector::new("Color:", COLORS).with_default(4),
    );
    let selections = app.run();
    if std::env::args_os().nth(1).is_some_and(|s| s == "--debug") {
        println!("{selections:#?}");
    } else if let Some(selections) = selections {
        for (k, v) in selections {
            match (k, v) {
                ("effects", Selection::Multi(efs)) if efs.is_empty() => {
                    println!("Effects: -- none --");
                }
                ("effects", Selection::Multi(efs)) => {
                    println!("Effects:");
                    for i in efs {
                        if i < EFFECTS.len() {
                            println!(" - {} ({i})", EFFECTS[i]);
                        } else {
                            println!(" - #{i}");
                        }
                    }
                }
                ("color", Selection::Radio(i)) if i < COLORS.len() => {
                    println!("Color: {} ({i})", COLORS[i]);
                }
                ("color", Selection::Radio(i)) => println!("Color: #{i}"),
                (k, v) => println!("UNEXPECTED OUTPUT: ({k:?}, {v:?})"),
            }
        }
    } else {
        println!("User quit");
    }
}
