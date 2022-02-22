use tokio::time::{self, Duration};

fn task_that_takes_a_second() {
    println!("hello");
}

pub async fn start() {
    println!("Timer started!");
    let mut interval = time::interval(time::Duration::from_secs(5));

    interval.tick().await;
    interval.tick().await;
    task_that_takes_a_second();
}
