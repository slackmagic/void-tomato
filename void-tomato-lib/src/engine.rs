use crate::tasks::Task;
use crate::timer::Timer;

pub struct Engine {
    focus_length: u64,
    short_break_length: u64,
    long_break_length: u64,
    break_interval: u8,
    tasks: Vec<Task>,
    state: State,
}

struct State {
    pub current_steps: u8,
    pub break_count: u8,
}

impl Engine {
    pub fn new(
        focus_length: u64,
        short_break_length: u64,
        long_break_length: u64,
        break_interval: u8,
    ) -> Engine {
        let state = State {
            current_steps: 1,
            break_count: 0,
        };

        Engine {
            focus_length,
            short_break_length,
            long_break_length,
            break_interval,
            tasks: Vec::new(),
            state: state,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub async fn next_focus(&mut self) {
        print!("Start step");
        self.state.current_steps += 1;

        let duration = match self.is_focus_step() {
            true => {
                println!(" >> FOCUS");
                self.focus_length
            }
            false => {
                self.state.break_count += 1;
                self.get_current_break_length()
            }
        };

        Timer::new(duration).start().await;
        println!("Step ended");
    }

    fn is_focus_step(&self) -> bool {
        self.state.current_steps % 2 == 0
    }

    fn get_current_break_length(&self) -> u64 {
        if self.state.break_count % self.break_interval == 0 {
            println!(" with long break !");
            self.long_break_length
        } else {
            println!(" with short break !");
            self.short_break_length
        }
    }
}
