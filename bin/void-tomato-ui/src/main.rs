use void_tomato_lib::timer;

#[tokio::main]
async fn main() {
    timer::start().await;
}
