mod task;
mod store;
mod create;
mod read;
mod upd;
mod delete;

use crate::store::Store;
use std::env;

fn ayuda() {
    println!("uso:");
    println!(" add <title> [desc] [due]");
    println!(" list [all|pending|doing|done]");
    println!(" update <title> <field> <value>");
    println!(" mark <title> <pending|doing|done>");
    println!(" delete <title>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        ayuda();
        return;
    }
    let cmd = args[1].as_str();
    let store = Store::new("tasks.json");
    match cmd {
        "add" => {
            if args.len() < 3 { println!("need title"); return; }
            let title = &args[2];
            let desc = if args.len() >= 4 { Some(args[3].as_str()) } else { None };
            let due = if args.len() >= 5 { Some(args[4].as_str()) } else { None };
            create::add(&store, title, desc, due);
        }
        "list" => {
            let mode = if args.len() >= 3 { args[2].as_str() } else { "pending" };
            read::list(&store, mode);
        }
        "update" => {
            if args.len() < 5 { println!("need title field value"); return; }
            upd::updat(&store, args[2].as_str(), args[3].as_str(), args[4].as_str());
        }
        "mark" => {
            if args.len() < 4 { println!("need title state"); return; }
            upd::mark(&store, args[2].as_str(), args[3].as_str());
        }
        "delete" => {
            if args.len() < 3 { println!("need title"); return; }
            delete::delet(&store, args[2].as_str());
        }
        _ => ayuda(),
    }
}