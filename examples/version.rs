extern crate curl;

fn main() {
    let ver = curl::Version::get();
    println!("{}", curl::Version::num());
    println!("{:?}", ver);
}
