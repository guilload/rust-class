use std::io::File;
use std::os;
use std::str;


fn main() {
    let args = os::args();

    if args.len() != 3 {
        println!("Usage: {} <secret> <ciphertext>", args[0]);
        return
    }

    let secret = File::open(&Path::new(args[1].as_slice())).read_to_end().ok().expect("foo!");
    let ciphertext = File::open(&Path::new(args[2].as_slice())).read_to_end().ok().expect("bar!");

    let plaintext = xor(&secret, &ciphertext);
    println!("{}", str::from_utf8(plaintext.as_slice()));
}

fn xor(right: &Vec<u8>, left: &Vec<u8>) -> Vec<u8> {
    let lenght = right.len();

    if lenght != left.len() {
        fail!();
    }

    let mut v: Vec<u8> = Vec::new();

    for i in range(0, lenght) {
        v.push(right[i] ^ left[i]);
    }

    v
}
