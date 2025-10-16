use crate::store::Store;

pub fn updat(store: &Store, title: &str, field: &str, val: &str) {
    let mut v = store.load();
    for t in &mut v {
        if t.title == title {
            match field {
                "title" => t.title = val.to_string(),
                "desc" => t.desc = Some(val.to_string()),
                "due" => t.due = Some(val.to_string()),
                _ => {},
            }
            store.save(&v);
            println!("updated {}", title);
            return;
        }
    }
    println!("not found {}", title);
}

pub fn mark(store: &Store, title: &str, st: &str) {
    let mut v = store.load();
    for t in &mut v {
        if t.title == title {
            if st == "done" || st == "doing" || st == "pending" {
                t.status = st.to_string();
                store.save(&v);
                println!("marked {}", title);
                return;
            }
        }
    }
    println!("not found {}", title);
}