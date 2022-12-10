pub mod caesar;
pub mod vignere;
pub mod euclid;
pub mod fiestel;
pub mod playfair;

fn main() {
    for i in 0..26 {
        println!("{}", caesar::decrypt("jfy ymj fuuqj", i, vignere::ALPHA_LOWER));
    }
}