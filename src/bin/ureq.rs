fn main() {
    for i in 0.. {
        println!("{i}");
        ureq::get("http://localhost:7777/").call().unwrap();
    }
}
