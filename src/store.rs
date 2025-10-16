use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::task::Task;

pub struct Store {
    path: String,
}

fn split_objs(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut depth: i32 = 0;
    let mut start: usize = 0;
    let bytes = s.as_bytes();
    let mut i: usize = 0;
    while i < s.len() {
        let c = bytes[i] as char;
        if c == '"' {
            let mut j = i + 1;
            while j < s.len() {
                let cc = bytes[j] as char;
                if cc == '\\' { j += 2; continue; }
                if cc == '"' { break; }
                j += 1;
            }
            i = j + 1;
            continue;
        }
        if c == '{' {
            if depth == 0 { start = i; }
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                out.push(s[start..=i].to_string());
            }
        }
        i += 1;
    }
    out
}

impl Store {
    pub fn new(p: &str) -> Store {
        let path = p.to_string();
        if !Path::new(&path).exists() {
            let mut f = File::create(&path).unwrap();
            f.write_all(b"[]").unwrap();
        }
        Store { path }
    }

    pub fn load(&self) -> Vec<Task> {
        let mut f = File::open(&self.path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let s = s.trim();
        if s.len() < 2 { return Vec::new(); }
        let objs = split_objs(s);
        let mut out: Vec<Task> = Vec::new();
        for o in objs {
            if let Some(t) = Task::from_json(&o) {
                out.push(t);
            }
        }
        out
    }

    pub fn save(&self, v: &[Task]) {
        let mut s = String::new();
        s.push('[');
        let mut first = true;
        for t in v {
            if !first { s.push(','); }
            s.push_str(&t.to_json());
            first = false;
        }
        s.push(']');
        let mut f = OpenOptions::new().write(true).truncate(true).open(&self.path).unwrap();
        f.write_all(s.as_bytes()).unwrap();
    }

    pub fn next_id(&self, v: &[Task]) -> u32 {
        let mut mx: u32 = 0;
        for t in v { if t.id > mx { mx = t.id; } }
        mx + 1
    }
}