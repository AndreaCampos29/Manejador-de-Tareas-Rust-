use crate::store::Store;

pub fn delet(store: &Store, title: &str) {
    let mut v = store.load();
    let orig = v.len();
    v.retain(|t| t.title != title);
    if v.len() == orig {
        println!("not found {}", title);
        return;
    }
    store.save(&v);
    println!("deleted {}", title);
}