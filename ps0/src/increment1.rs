fn increment(p: &mut Vec<int>) {
    for x in p.iter_mut() {
        *x += 1;
    }
}

fn main() {
    let mut p = vec![1i, 2i, 3i];
    increment(&mut p);

    for x in p.iter() {
        print!("{} ", x);
   }
}
