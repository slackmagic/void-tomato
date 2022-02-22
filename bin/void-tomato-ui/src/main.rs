use notify_rust::{Hint, Notification};
use void_tomato_lib::timer;

#[tokio::main]
async fn main() {
    timer::start().await;
    Notification::new()
        .summary("🍅 Category:email")
        .body(
            "This has nothing to do with emails.\nIt should not go away until you acknowledge it.",
        )
        .icon("thunderbird")
        .appname("thunderbird")
        .timeout(0) // this however is
        .show();
}
