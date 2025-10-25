use std::fs;
use task_tracker::{store::Store, create};

#[test]
fn list_all() {
    let path = "test_list.json";
    let s = Store::new(path);
    create::add(&s, "t1", None, None);
    create::add(&s, "t2", None, None);
    let v = s.load();
    assert!(v.len() >= 2);
    fs::remove_file(path).ok();
}