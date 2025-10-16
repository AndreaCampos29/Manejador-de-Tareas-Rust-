use crate::task::Task;
use crate::store::Store;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn add(store: &Store, title: &str, desc: Option<&str>, due: Option<&str>) {
    let mut v = store.load();
    let id = store.next_id(&v);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let created = format!("{}", now);
    let t = Task {
        id,
        title: title.to_string(),
        desc: desc.map(|s| s.to_string()),
        status: "pending".to_string(),
        created,
        due: due.map(|s| s.to_string()),
    };
    v.push(t);
    store.save(&v);
    println!("added {}", id);
}