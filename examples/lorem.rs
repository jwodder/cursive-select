use cursivelect::{Form, MultiSelector, RadioSelector, Selection};

const LOREM: [&str; 7] = [
    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod",
    "tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim ad minim veniam,",
    "quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo",
    "consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse",
    "molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan",
    "et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore",
    "te feugait nulla facilisi.  Nam liber tempor cum soluta nobis eleifend option congue",
];

const IPSUM: [&str; 7] = [
    "nihil imperdiet doming id quod mazim placerat facer possim assum.  Typi non habent",
    "claritatem insitam; est usus legentis in iis qui facit eorum claritatem.  Investigationes",
    "demonstraverunt lectores legere me lius quod ii legunt saepius.  Claritas est etiam",
    "processus dynamicus, qui sequitur mutationem consuetudium lectorum.  Mirum est notare",
    "quam littera gothica, quam nunc putamus parum claram, anteposuerit litterarum formas",
    "humanitatis per seacula quarta decima et quinta decima.  Eodem modo typi, qui nunc",
    "nobis videntur parum clari, fiant sollemnes in futurum.",
];

fn main() {
    let mut app = Form::new();
    app.add("lorem", RadioSelector::new("Lorem", LOREM));
    app.add("ipsum", MultiSelector::new("Ipsum", IPSUM));
    let selections = app.run();
    if std::env::args_os().nth(1).is_some_and(|s| s == "--debug") {
        println!("{selections:#?}");
    } else if let Some(selections) = selections {
        for (k, v) in selections {
            match (k, v) {
                ("lorem", Selection::Radio(i)) if i < LOREM.len() => {
                    println!("Lorem: {} ({i})", LOREM[i]);
                }
                ("lorem", Selection::Radio(i)) => println!("Lorem: #{i}"),
                ("ipsum", Selection::Multi(ips)) if ips.is_empty() => {
                    println!("Ipsum: -- none --");
                }
                ("ipsum", Selection::Multi(ips)) => {
                    println!("Ipsum:");
                    for i in ips {
                        if i < IPSUM.len() {
                            println!(" - {} ({i})", IPSUM[i]);
                        } else {
                            println!(" - #{i}");
                        }
                    }
                }
                (k, v) => println!("UNEXPECTED OUTPUT: ({k:?}, {v:?})"),
            }
        }
    } else {
        println!("User quit");
    }
}
