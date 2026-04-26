use cursive::{
    Cursive, View,
    event::{Event, EventResult, Key},
    traits::Finder,
    view::Nameable,
    views::{
        Checkbox, Dialog, DialogFocus, DummyView, LinearLayout, NamedView, OnEventView, PaddedView,
        RadioGroup, ScrollView, TextView,
    },
};
use itertools::{Itertools, Position};
use mitsein::vec1::Vec1;
use std::collections::BTreeSet;

const OPTION_INDENT: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Curselect<T> {
    selectors: Vec<(T, Selector)>,
}

impl<T: 'static> Curselect<T> {
    pub fn new() -> Self {
        Curselect {
            selectors: Vec::new(),
        }
    }

    pub fn add(&mut self, key: T, selector: Selector) {
        self.selectors.push((key, selector));
    }

    pub fn run(self) -> Option<Vec<(T, Selection)>> {
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
        for (pos, (si, sel)) in selectors.into_iter().enumerate().with_position() {
            if si > 0 {
                layout.add_child(DummyView);
            }
            let mut sublayout = LinearLayout::vertical();
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
                    for (i, opt) in options.into_iter().enumerate() {
                        let mut button = group.button(i, opt);
                        if i == default {
                            let _ = button.select();
                        }
                        if i == 0 {
                            match pos {
                                Position::Only => sublayout
                                    .add_child(button.with_name("top").with_name("bottom-target")),
                                Position::First => sublayout.add_child(button.with_name("top")),
                                Position::Last => {
                                    sublayout.add_child(button.with_name("bottom-target"));
                                }
                                Position::Middle => sublayout.add_child(button),
                            }
                        } else {
                            sublayout.add_child(button);
                        }
                    }
                }
                Selector::Multi { title, options } => {
                    layout.add_child(TextView::new(title));
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
                        let mut row = LinearLayout::horizontal();
                        if i == 0 {
                            match pos {
                                Position::Only => row
                                    .add_child(chkbox.with_name("top").with_name("bottom-target")),
                                Position::First => row.add_child(chkbox.with_name("top")),
                                Position::Last => row.add_child(chkbox.with_name("bottom-target")),
                                Position::Middle => row.add_child(chkbox),
                            }
                        } else {
                            row.add_child(chkbox);
                        }
                        row.add_child(DummyView);
                        row.add_child(TextView::new(opt));
                        sublayout.add_child(row);
                    }
                }
            }
            layout.add_child(PaddedView::lrtb(
                OPTION_INDENT,
                0,
                0,
                0,
                sublayout.with_name("sublayout"),
            ));
        }
        siv.add_layer(
            OnEventView::new(
                Dialog::around(ScrollView::new(layout.with_name("layout")).with_name("scrollview"))
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
            .on_pre_event_inner(Key::Home, |dialog, _| focus_top(dialog))
            .on_pre_event_inner(Key::End, |dialog, _| {
                dialog.set_focus(DialogFocus::Button(0));
                Some(EventResult::Consumed(None))
            })
            .on_pre_event_inner(Key::Tab, |dialog, _| match dialog.focus() {
                DialogFocus::Content => {
                    if dialog.call_on_name("layout", |layout: &mut LinearLayout| {
                        for i in (layout.get_focus_index() + 1)..layout.len() {
                            if layout.set_focus_index(i).is_ok() {
                                // Rather than trying to set the inner focus of
                                // the just-focused view to its first element,
                                // which is tricky, we just set the inner focus
                                // of every options block.
                                layout.call_on_all(
                                    &cursive::view::Selector::Name("sublayout"),
                                    |sublayout: &mut LinearLayout| {
                                        let _ = sublayout.set_focus_index(0);
                                    },
                                );
                                return true;
                            }
                        }
                        false
                    }) == Some(true)
                    {
                        let _ = dialog.call_on_name(
                            "scrollview",
                            |scr: &mut ScrollView<NamedView<LinearLayout>>| {
                                scr.scroll_to_important_area()
                            },
                        );
                    } else {
                        dialog.set_focus(DialogFocus::Button(0));
                    }
                    Some(EventResult::Consumed(None))
                }
                DialogFocus::Button(0) => {
                    dialog.set_focus(DialogFocus::Button(1));
                    Some(EventResult::Consumed(None))
                }
                DialogFocus::Button(1) => focus_top(dialog),
                DialogFocus::Button(_) => unreachable!(),
            })
            .on_pre_event_inner(Event::Shift(Key::Tab), |dialog, _| match dialog.focus() {
                DialogFocus::Content => {
                    if dialog.call_on_name("layout", |layout: &mut LinearLayout| {
                        for i in (0..layout.get_focus_index()).rev() {
                            if layout.set_focus_index(i).is_ok() {
                                // Rather than trying to set the inner focus of
                                // the just-focused view to its first element,
                                // which is tricky, we just set the inner focus
                                // of every options block.
                                layout.call_on_all(
                                    &cursive::view::Selector::Name("sublayout"),
                                    |sublayout: &mut LinearLayout| {
                                        let _ = sublayout.set_focus_index(0);
                                    },
                                );
                                return true;
                            }
                        }
                        false
                    }) == Some(true)
                    {
                        let _ = dialog.call_on_name(
                            "scrollview",
                            |scr: &mut ScrollView<NamedView<LinearLayout>>| {
                                scr.scroll_to_important_area()
                            },
                        );
                    } else {
                        dialog.set_focus(DialogFocus::Button(1));
                    }
                    Some(EventResult::Consumed(None))
                }
                DialogFocus::Button(0) => focus_bottom(dialog),
                DialogFocus::Button(1) => {
                    dialog.set_focus(DialogFocus::Button(0));
                    Some(EventResult::Consumed(None))
                }
                DialogFocus::Button(_) => unreachable!(),
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

impl<T: 'static> Default for Curselect<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State<T> {
    outcome: Vec<(T, Selection)>,
    ok: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selector {
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
pub enum Selection {
    Single(usize),
    Multi(BTreeSet<usize>),
}

#[allow(clippy::unnecessary_wraps)]
fn focus_top(dialog: &mut Dialog) -> Option<EventResult> {
    dialog.set_focus(DialogFocus::Content);
    if let Some(scroller) = dialog
        .get_content_mut()
        .as_any_mut()
        .downcast_mut::<ScrollView<LinearLayout>>()
    {
        scroller.scroll_to_top();
    }
    let cb = if let Ok(EventResult::Consumed(val)) =
        dialog.focus_view(&cursive::view::Selector::Name("top"))
    {
        val
    } else {
        None
    };
    Some(EventResult::Consumed(cb))
}

#[allow(clippy::unnecessary_wraps)]
fn focus_bottom(dialog: &mut Dialog) -> Option<EventResult> {
    dialog.set_focus(DialogFocus::Content);
    if let Some(scroller) = dialog
        .get_content_mut()
        .as_any_mut()
        .downcast_mut::<ScrollView<LinearLayout>>()
    {
        scroller.scroll_to_bottom();
    }
    let cb = if let Ok(EventResult::Consumed(val)) =
        dialog.focus_view(&cursive::view::Selector::Name("bottom-target"))
    {
        val
    } else {
        None
    };
    Some(EventResult::Consumed(cb))
}
