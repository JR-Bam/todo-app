mod todo_func;

fn main() {
    let mut storage = todo_func::read_state_from_file().unwrap_or_default();
    storage.list.insert("Bambibfsnjfsdfndsmnam".to_string(), false);

    todo_func::save_state_to_file(&storage).expect("Failed to save state");
}
