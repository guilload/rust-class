 use std::os;


fn collatz(n: int) -> int {
    if n == 1 { return 0; }

    match n % 2 {
        0 => { 1 + collatz(n / 2) }
        _ => { 1 + collatz(n * 3 + 1) }
    }
}


fn main() {
    let args = os::args();
    let mut i = 1;

    if args.len() < 2 {
        println!("error: please provide a number as argument");
        return
    }

    let steps: int = from_str(args[1].as_slice()).unwrap();

    while collatz(i) != steps {
        i += 1;
    }

    println!("The lowest number requiring {} Collatz steps is {}", steps, i);
}
