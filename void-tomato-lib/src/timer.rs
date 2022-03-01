use tokio::time::{self, Duration};

pub struct Timer {
    duration: u64,
}

impl Timer {
    pub fn new(duration: u64) -> Timer {
        Timer { duration }
    }
    pub async fn start(self) {
        println!("__Timer started!");
        let mut interval = time::interval(time::Duration::from_secs(self.duration));
        interval.tick().await;
        interval.tick().await;
        println!("__Timer stopped!");
    }
}
