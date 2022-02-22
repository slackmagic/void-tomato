use crate::tasks::Task;

pub struct Pomodoro {
    pause_length: u8,
    pause_long_length: u8,
    pause_steps: u8,
    tasks: Vec<Task>,
}

impl Pomodoro {
    pub fn new(pause_length: u8, pause_steps: u8, pause_long_length: u8) -> Pomodoro {
        Pomodoro {
            pause_length,
            pause_steps,
            pause_long_length,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn start() {
        println!("Pomodoro started!");
    }
}
