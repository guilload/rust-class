fn increment(p: Vec<int>) -> Vec<int> {
    let mut q: Vec<int> = Vec::new();

    for &x in p.iter() {
        q.push(x + 1);
    }

    q
}

fn main() {
    let p = vec![1i, 2i, 3i];
    let q = increment(p);

    for &x in q.iter() {
        print!("{} ", x);
   }
}
