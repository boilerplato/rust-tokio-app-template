use tokio::task;

async fn print_hello() {
    task::spawn(async {
        println!("Hello, world!");
    })
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    print_hello().await;
}
