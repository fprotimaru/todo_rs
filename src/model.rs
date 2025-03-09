#[derive(Debug)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String, description: String, completed: bool) -> Self {
        Self {
            id: None,
            title,
            description,
            completed,
        }
    }
}
