use std::fs;
use task_tracker::{store::Store, create, upd};

#[test]
fn update_title() {
    let path = "test_upd.json";
    let s = Store::new(path);
    create::add(&s, "old title", None, None);
    upd::updat(&s, "old title", "title", "new title");
    let v = s.load();
    assert!(v.iter().any(|t| t.title == "new title"));
    fs::remove_file(path).ok();
}

#[test]
fn mark_done() {
    let path = "test_mark.json";
    let s = Store::new(path);
    create::add(&s, "do something", None, None);
    upd::mark(&s, "do something", "done");
    let v = s.load();
    assert!(v.iter().any(|t| t.status == "done"));
    fs::remove_file(path).ok();
}