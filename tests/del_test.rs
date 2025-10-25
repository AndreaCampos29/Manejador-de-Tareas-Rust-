use std::fs;
use task_tracker::{store::Store, create, delete};

#[test]
fn delete_task() {
    let path = "test_del.json";
    let s = Store::new(path);
    create::add(&s, "temp", None, None);
    delete::delet(&s, "temp");
    let v = s.load();
    assert!(!v.iter().any(|t| t.title == "temp"));
    fs::remove_file(path).ok();
}