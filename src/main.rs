use ring::rand::{self, SecureRandom};

fn main() {
    println!("Hello, world!");
    let rng = rand::SystemRandom::new();
    let mut dest = [0u8; 32];
    let val = rng.fill(&mut dest);
    print!("{:?}", val)
}
