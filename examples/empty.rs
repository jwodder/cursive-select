use cursivelect::Form;

fn main() {
    let app = Form::<i32>::new();
    let selections = app.run();
    println!("{selections:#?}");
}
