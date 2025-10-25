use std::fs;
use task_tracker::{store::Store, create};

#[test]
fn add_task() {
    let path = "test_add.json";
    let s = Store::new(path);
    create::add(&s, "test add", Some("desc test"), Some("2025-12-31"));
    let v = s.load();
    assert!(v.iter().any(|t| t.title == "test add"));
    fs::remove_file(path).ok();
}