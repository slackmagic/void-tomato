pub struct Task {
    name: String,
    category: String,
}

impl Task {
    pub fn new(name: String, category: String) -> Task {
        Task { name, category }
    }
}
