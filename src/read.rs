use crate::store::Store;

pub fn list(store: &Store, mode: &str) {
    let v = store.load();
    for t in v {
        let show = match mode {
            "all" => true,
            "pending" => t.status == "pending",
            "done" => t.status == "done",
            "doing" => t.status == "doing",
            _ => true,
        };
        if show {
            let mut due = String::new();
            if let Some(d) = t.due { due = format!(" due:{}", d); }
            let mut desc = String::new();
            if let Some(d) = t.desc { desc = format!(" - {}", d); }
            println!("[{}] {} ({}){}{}", t.id, t.title, t.status, desc, due);
        }
    }
}