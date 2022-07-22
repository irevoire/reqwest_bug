fn main() {
    for i in 0.. {
        println!("{i}");
        reqwest::blocking::get("http://localhost:7777/").unwrap();
    }
}
