#![expect(clippy::unwrap_used)]
use cursive::{
    Cursive,
    event::Key,
    views::{Button, Checkbox, CircularFocus, LinearLayout, PaddedView, RadioGroup, TextView},
};
use mitsein::vec1::{Vec1, vec1};
use std::collections::BTreeSet;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering::SeqCst},
};

const OPTION_INDENT: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Curselect<T> {
    selectors: Vec<(T, Selector)>,
}

impl<T: Clone + Send + Sync + 'static> Curselect<T> {
    fn new() -> Self {
        Curselect {
            selectors: Vec::new(),
        }
    }

    fn add(&mut self, key: T, selector: Selector) {
        self.selectors.push((key, selector));
    }

    fn run(self) -> Option<Vec<(T, Selection)>> {
        let mut outcome = Vec::with_capacity(self.selectors.len());
        let mut selectors = Vec::with_capacity(self.selectors.len());
        for (key, s) in self.selectors {
            let keyout = match &s {
                Selector::Single { default, .. } => Selection::Single(*default),
                Selector::Multi { .. } => Selection::Multi(BTreeSet::new()),
            };
            outcome.push((key, keyout));
            selectors.push(s);
        }
        let outcome = Arc::new(Mutex::new(outcome));
        let mut siv = cursive::default();
        siv.add_global_callback('q', Cursive::quit);
        siv.add_global_callback('Q', Cursive::quit);
        siv.add_global_callback(Key::Esc, Cursive::quit);
        let mut layout = LinearLayout::vertical();
        for (si, sel) in selectors.iter().enumerate() {
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
                    for (i, opt) in options.iter().enumerate() {
                        let chkbox = Checkbox::new().on_change({
                            let outcome = Arc::clone(&outcome);
                            move |_, checked| {
                                let mut oc = outcome.lock().unwrap();
                                let Selection::Multi(ref mut selection) = oc[si].1 else {
                                    unreachable!();
                                };
                                if checked {
                                    selection.insert(i);
                                } else {
                                    selection.remove(&i);
                                }
                            }
                        });
                        let lbl = TextView::new(format!(" {opt}"));
                        let row = LinearLayout::horizontal().child(chkbox).child(lbl);
                        sublayout.add_child(row);
                    }
                    layout.add_child(PaddedView::lrtb(OPTION_INDENT, 0, 0, 0, sublayout));
                }
            }
        }
        let ok = Arc::new(AtomicBool::new(false));
        let ok_button = Button::new("OK", {
            let ok = Arc::clone(&ok);
            move |s| {
                ok.store(true, SeqCst);
                s.quit();
            }
        });
        let cancel_button = Button::new("Cancel", Cursive::quit);
        layout.add_child(
            LinearLayout::horizontal()
                .child(ok_button)
                .child(cancel_button),
        );
        siv.add_layer(CircularFocus::new(layout).wrap_up_down().wrap_tab());
        siv.run();
        ok.load(SeqCst).then(|| outcome.lock().unwrap().clone())
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
    Multi(BTreeSet<usize>),
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
