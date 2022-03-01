use notify_rust::{Hint, Notification};
use void_tomato_lib::engine::Engine;
use void_tomato_lib::tasks::Task;

#[tokio::main]
async fn main() {
    let mut engine: Engine = Engine::new(3, 1, 5, 4);
    engine.add_task(Task::new("Task".to_string(), "Category".to_string()));
    for _i in 0..10 {
        println!("==========================");
        engine.next_focus().await;
        //alert();
    }
}

fn alert() {
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
