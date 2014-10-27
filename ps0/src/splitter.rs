use std::io::File;
use std::os;
use std::rand::random;


fn main() {
    let args = os::args();

    if args.len() != 2 {
        println!("Usage: {} <inputfile>", args[0]);
        return
    }

    let filepath = &args[1];
    let plaintext = File::open(&Path::new(filepath.as_slice())).read_to_end().ok().expect("foo!");
    let (secret, ciphertext): (Vec<u8>, Vec<u8>) = otp(&plaintext);

    let mut secretf = File::create(&Path::new(filepath + ".secret")).ok().expect("bar!");
    let mut ciphertextf = File::create(&Path::new(filepath + ".ciphertext")).ok().expect("baz!");

    secretf.write(secret.as_slice());
    ciphertextf.write(ciphertext.as_slice());
}

fn otp(plaintext: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut secret: Vec<u8> = Vec::new();

    for _ in range(0, plaintext.len()) {
        secret.push(random());
    }

    let ciphertext = xor(plaintext, &secret);
    (secret, ciphertext)
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
