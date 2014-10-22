
fn main() {
    let x = (51i, true);

    match x {
        (20...26, true) => println!("a"),
        (_, true) => println!("b"),
        (40...49, _) => println!("c"),
        (_, _) => println!("d"),
    }
}
