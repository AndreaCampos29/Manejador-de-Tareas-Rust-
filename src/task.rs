pub struct Task {
    pub id: u32,
    pub title: String,
    pub desc: Option<String>,
    pub status: String,
    pub created: String,
    pub due: Option<String>,
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl Task {
    pub fn to_json(&self) -> String {
        let mut parts = Vec::new();
        parts.push(format!("\"id\":{}", self.id));
        parts.push(format!("\"title\":\"{}\"", esc(&self.title)));
        if let Some(d) = &self.desc {
            parts.push(format!("\"desc\":\"{}\"", esc(d)));
        }
        parts.push(format!("\"status\":\"{}\"", esc(&self.status)));
        parts.push(format!("\"created\":\"{}\"", esc(&self.created)));
        if let Some(dd) = &self.due {
            parts.push(format!("\"due\":\"{}\"", esc(dd)));
        }
        format!("{{{}}}", parts.join(","))
    }

    pub fn from_json(s: &str) -> Option<Task> {
        fn get_str(s: &str, k: &str) -> Option<String> {
            let pat = format!("\"{}\"", k);
            let mut i = 0usize;
            while let Some(p) = s[i..].find(&pat) {
                i += p + pat.len();
                if let Some(col) = s[i..].find(':') {
                    i += col + 1;
                    while s[i..].starts_with(char::is_whitespace) { i += 1; }
                    if s[i..].starts_with('"') {
                        i += 1;
                        let mut out = String::new();
                        let mut j = i;
                        let mut esc = false;
                        while j < s.len() {
                            let c = s.as_bytes()[j] as char;
                            if esc {
                                out.push(c);
                                esc = false;
                            } else if c == '\\' {
                                esc = true;
                            } else if c == '"' {
                                return Some(out);
                            } else {
                                out.push(c);
                            }
                            j += 1;
                        }
                        return None;
                    } else {
                        return None;
                    }
                }
            }
            None
        }
        fn get_num(s: &str, k: &str) -> Option<u32> {
            let pat = format!("\"{}\"", k);
            let mut i = 0usize;
            while let Some(p) = s[i..].find(&pat) {
                i += p + pat.len();
                if let Some(col) = s[i..].find(':') {
                    i += col + 1;
                    while s[i..].starts_with(char::is_whitespace) { i += 1; }
                    let mut j = i;
                    let bytes = s.as_bytes();
                    while j < s.len() {
                        let c = bytes[j] as char;
                        if c.is_ascii_digit() { j += 1; } else { break; }
                    }
                    if j > i {
                        if let Ok(v) = s[i..j].parse::<u32>() {
                            return Some(v);
                        }
                    }
                }
            }
            None
        }
        let id = get_num(s, "id")?;
        let title = get_str(s, "title")?;
        let desc = get_str(s, "desc");
        let status = get_str(s, "status")?;
        let created = get_str(s, "created")?;
        let due = get_str(s, "due");
        Some(Task { id, title, desc, status, created, due })
    }
}