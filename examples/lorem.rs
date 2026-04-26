use cursivelect::{Curselect, Selector};
use mitsein::vec1::vec1;

fn main() {
    let mut app = Curselect::new();
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
    let selections = app.run();
    println!("{selections:#?}");
}
