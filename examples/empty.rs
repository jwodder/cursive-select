use cursivelect::Curselect;

fn main() {
    let app = Curselect::<i32>::new();
    let selections = app.run();
    println!("{selections:#?}");
}
