#[tokio::main]
async fn main() {
    for i in 0.. {
        println!("{i}");
        reqwest::get("http://localhost:7777/").await.unwrap();
    }
}
