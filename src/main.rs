use cursive::{
    Cursive, View,
    direction::Direction,
    event::{EventResult, Key},
    view::{Finder, Nameable},
    views::{
        Checkbox, Dialog, DialogFocus, DummyView, LinearLayout, OnEventView, PaddedView,
        RadioGroup, ScrollView, TextView,
    },
};
use mitsein::vec1::{Vec1, vec1};
use std::collections::BTreeSet;

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
        let mut siv = cursive::default();
        siv.set_user_data(State { outcome, ok: false });
        siv.with_theme(|theme| {
            theme.shadow = false;
        });
        siv.add_global_callback('q', Cursive::quit);
        siv.add_global_callback('Q', Cursive::quit);
        siv.add_global_callback(Key::Esc, Cursive::quit);
        siv.add_global_callback('h', |s| s.on_event(Key::Left.into()));
        siv.add_global_callback('j', |s| s.on_event(Key::Down.into()));
        siv.add_global_callback('k', |s| s.on_event(Key::Up.into()));
        siv.add_global_callback('l', |s| s.on_event(Key::Right.into()));
        siv.add_global_callback('w', |s| s.on_event(Key::PageUp.into()));
        siv.add_global_callback('z', |s| s.on_event(Key::PageDown.into()));
        siv.add_global_callback('g', |s| s.on_event(Key::Home.into()));
        siv.add_global_callback('G', |s| s.on_event(Key::End.into()));
        let mut layout = LinearLayout::vertical();
        for (si, sel) in selectors.iter().enumerate() {
            if si > 0 {
                layout.add_child(DummyView);
            }
            match sel {
                Selector::Single {
                    title,
                    options,
                    default,
                } => {
                    layout.add_child(TextView::new(title));
                    let mut group = RadioGroup::<usize>::new().on_change({
                        move |s, &radioed| {
                            s.with_user_data(|st: &mut State<T>| {
                                st.outcome[si].1 = Selection::Single(radioed);
                            });
                        }
                    });
                    let mut sublayout = LinearLayout::vertical();
                    for (i, opt) in options.iter().enumerate() {
                        let mut button = group.button(i, opt);
                        if i == *default {
                            let _ = button.select();
                        }
                        sublayout.add_child(button);
                    }
                    let padded = PaddedView::lrtb(OPTION_INDENT, 0, 0, 0, sublayout);
                    if si == 0 {
                        layout.add_child(padded.with_name("top"));
                    } else {
                        layout.add_child(padded);
                    }
                }
                Selector::Multi { title, options } => {
                    layout.add_child(TextView::new(title));
                    let mut sublayout = LinearLayout::vertical();
                    for (i, opt) in options.iter().enumerate() {
                        let chkbox = Checkbox::new().on_change({
                            move |s, checked| {
                                s.with_user_data(|st: &mut State<T>| {
                                    let Selection::Multi(ref mut selection) = st.outcome[si].1
                                    else {
                                        unreachable!();
                                    };
                                    if checked {
                                        selection.insert(i);
                                    } else {
                                        selection.remove(&i);
                                    }
                                });
                            }
                        });
                        let lbl = TextView::new(opt);
                        if si == 0 && i == 0 {
                            sublayout.add_child(
                                LinearLayout::horizontal()
                                    .child(chkbox.with_name("top"))
                                    .child(DummyView)
                                    .child(lbl),
                            );
                        } else {
                            sublayout.add_child(
                                LinearLayout::horizontal()
                                    .child(chkbox)
                                    .child(DummyView)
                                    .child(lbl),
                            );
                        }
                    }
                    let padded = PaddedView::lrtb(OPTION_INDENT, 0, 0, 0, sublayout);
                    if si == 0 {
                        layout.add_child(padded.with_name("top"));
                    } else {
                        layout.add_child(padded);
                    }
                }
            }
        }
        siv.add_layer(
            OnEventView::new(
                Dialog::around(ScrollView::new(layout))
                    .button("OK", {
                        move |s| {
                            s.with_user_data(|st: &mut State<T>| {
                                st.ok = true;
                            });
                            s.quit();
                        }
                    })
                    .button("Cancel", Cursive::quit),
            )
            .on_pre_event_inner(Key::Home, |dialog, _| {
                dialog.set_focus(DialogFocus::Content);
                let scroller = dialog
                    .get_content_mut()
                    .as_any_mut()
                    .downcast_mut::<ScrollView<LinearLayout>>()
                    .expect("dialog content should be ScrollView");
                scroller.scroll_to_top();
                dialog.call_on_name("top", |b: &mut PaddedView<LinearLayout>| {
                    b.take_focus(Direction::up())
                });
                Some(EventResult::Consumed(None))
            })
            .on_pre_event_inner(Key::End, |dialog, _| {
                dialog.set_focus(DialogFocus::Button(0));
                Some(EventResult::Consumed(None))
            }),
        );
        siv.run();
        match siv.take_user_data() {
            Some(State { ok: true, outcome }) => Some(outcome),
            Some(State { ok: false, .. }) => None,
            None => panic!("Could not get user data back"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State<T> {
    outcome: Vec<(T, Selection)>,
    ok: bool,
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
    let arg = std::env::args_os().nth(1);
    let arg = arg.as_ref();
    if arg.is_some_and(|s| s == "full") {
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
    } else if arg.is_some_and(|s| s == "lorem") {
        app.add(
            "lorem",
            Selector::Single {
                title: "Lorem".to_owned(),
                options: vec1![
                    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod".into(),
                    "tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim ad minim veniam,".into(),
                    "quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo".into(),
                    "consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse".into(),
                    "molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan".into(),
                    "et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore".into(),
                    "te feugait nulla facilisi.  Nam liber tempor cum soluta nobis eleifend option congue".into(),
                ],
                default: 0,
            }
        );
        app.add("ipsum",
            Selector::Multi {
                title: "Ipsum".to_owned(),
                options: vec1![
                    "nihil imperdiet doming id quod mazim placerat facer possim assum.  Typi non habent".into(),
                    "claritatem insitam; est usus legentis in iis qui facit eorum claritatem.  Investigationes".into(),
                    "demonstraverunt lectores legere me lius quod ii legunt saepius.  Claritas est etiam".into(),
                    "processus dynamicus, qui sequitur mutationem consuetudium lectorum.  Mirum est notare".into(),
                    "quam littera gothica, quam nunc putamus parum claram, anteposuerit litterarum formas".into(),
                    "humanitatis per seacula quarta decima et quinta decima.  Eodem modo typi, qui nunc".into(),
                    "nobis videntur parum clari, fiant sollemnes in futurum.".into(),
                ],
            }
        );
    } else if arg.is_some_and(|s| s == "empty") {
    } else {
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
    }
    let selections = app.run();
    println!("{selections:#?}");
}
