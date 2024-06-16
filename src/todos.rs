use std::{fmt::Display, io::Write, path::Path};

#[derive(Debug)]
pub struct Todo {
    id: usize,
    is_complete: bool,
    text: String,
}
impl Todo {
    pub fn from_csv_line(csv_line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut todo = Todo {
            id: 0,
            is_complete: false,
            text: String::new(),
        };
        for (column_index, column_value) in csv_line.split(',').enumerate() {
            match column_index {
                0 => todo.id = column_value.parse()?,
                1 => todo.is_complete = column_value.parse()?,
                2 => todo.text = column_value.replace("\"", "").to_owned(),
                _ => Err(format!("too many columns on line"))?,
            }
        }
        return Ok(todo);
    }
    pub fn to_csv_line(&self) -> String {
        return format!("{},{},\"{}\"\n", self.id, self.is_complete, self.text);
    }
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{} is {}\n{}\n",
            self.id,
            if self.is_complete { "done" } else { "not done" },
            self.text
        )?;
        return Ok(());
    }
}

#[derive(Debug, Default)]
pub struct Todos {
    next_id: usize,
    list: Vec<Todo>,
}
impl Todos {
    pub fn from_csv_file(csv_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let csv = std::fs::read_to_string(csv_path)?;
        return Self::from_csv(&csv);
    }
    pub fn from_csv(csv: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut todos = Todos::default();
        for line in csv.lines() {
            todos.list.push(Todo::from_csv_line(line)?);
        }
        todos.next_id = todos
            .list
            .iter()
            .max_by_key(|todo| todo.id)
            .map(|todo| todo.id + 1)
            .unwrap_or(0);
        return Ok(todos);
    }
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        for todo in self.list.iter() {
            csv.push_str(&todo.to_csv_line());
        }
        return csv;
    }
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::options()
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(self.to_csv().as_bytes())?;
        return Ok(());
    }

    pub fn add_todos(&mut self, todos: Vec<String>) {
        for text in todos.into_iter() {
            let todo = Todo {
                id: {
                    let id = self.next_id;
                    self.next_id += 1;
                    id
                },
                is_complete: false,
                text,
            };
            self.list.push(todo);
        }
    }
    pub fn remove_todo(&mut self, todo_id: usize) {
        self.list.retain(|todo| todo.id != todo_id);
    }
    pub fn sort_by_id(&mut self) {
        self.list.sort_by_key(|todo| todo.id);
    }
    pub fn sort_by_is_complete(&mut self) {
        self.list.sort_by_key(|todo| todo.is_complete);
    }
    pub fn complete_todo(&mut self, todo_id: usize) {
        self.sort_by_id();
        let Ok(todo_index) = self.list.binary_search_by_key(&todo_id, |todo| todo.id) else {
            return;
        };
        self.list[todo_index].is_complete = true;
    }
    pub fn all_completed_todos(&self) -> impl Iterator<Item = &Todo> {
        return self.list.iter().filter(|todo| todo.is_complete);
    }
    pub fn all_incomplete_todos(&self) -> impl Iterator<Item = &Todo> {
        return self.list.iter().filter(|todo| !todo.is_complete);
    }
}
impl Display for Todos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for todo in self.list.iter() {
            write!(f, "{}\n", todo)?;
        }
        return Ok(());
    }
}
