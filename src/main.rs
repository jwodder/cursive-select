use cursive::{
    Cursive,
    event::Key,
    views::{Checkbox, LinearLayout, PaddedView, RadioGroup, TextView},
};
use mitsein::vec1::{Vec1, vec1};

const OPTION_INDENT: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Curselect<T> {
    selectors: Vec<(T, Selector)>,
}

impl<T> Curselect<T> {
    fn new() -> Self {
        Curselect {
            selectors: Vec::new(),
        }
    }

    fn add(&mut self, key: T, selector: Selector) {
        self.selectors.push((key, selector));
    }

    fn run(self) -> Option<Vec<(T, Selection)>> {
        if self.selectors.is_empty() {
            return Some(Vec::new());
            // Or return None?
            // Or show a screen with just "OK" and "Cancel" buttons?
        }
        let mut siv = cursive::default();
        siv.add_global_callback('q', Cursive::quit);
        siv.add_global_callback('Q', Cursive::quit);
        siv.add_global_callback(Key::Esc, Cursive::quit);
        let mut layout = LinearLayout::vertical();
        for (_, sel) in &self.selectors {
            match sel {
                Selector::Single {
                    title,
                    options,
                    default,
                } => {
                    layout.add_child(TextView::new(title));
                    let mut group = RadioGroup::new();
                    let mut sublayout = LinearLayout::vertical();
                    for (i, opt) in options.iter().enumerate() {
                        let mut button = group.button(i, opt);
                        if i == *default {
                            let _ = button.select();
                        }
                        sublayout.add_child(button);
                    }
                    layout.add_child(PaddedView::lrtb(OPTION_INDENT, 0, 0, 0, sublayout));
                }
                Selector::Multi { title, options } => {
                    layout.add_child(TextView::new(title));
                    let mut sublayout = LinearLayout::vertical();
                    for opt in options {
                        let chkbox = Checkbox::new();
                        let lbl = TextView::new(format!(" {opt}"));
                        let row = LinearLayout::horizontal().child(chkbox).child(lbl);
                        sublayout.add_child(row);
                    }
                    layout.add_child(PaddedView::lrtb(OPTION_INDENT, 0, 0, 0, sublayout));
                }
            }
        }
        siv.add_layer(layout);
        siv.run();
        None // TODO: Return selections
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Selector {
    Single {
        title: String,
        options: Vec1<String>,
        default: usize,
    },
    Multi {
        title: String,
        options: Vec1<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Selection {
    Single(usize),
    Multi(Vec<usize>),
}

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
