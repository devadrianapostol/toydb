use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Debug)]
pub struct DB {
    data: HashMap<String, String>,
    writer: BufWriter<File>,
    path: String,
}

#[derive(Debug, PartialEq)]
enum Op {
    Put { key: String, value: String },
    // Del later
}

impl DB {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)?;

        let mut data = HashMap::new();
        let reader = BufReader::new(File::open(&path)?);

        for line in reader.lines() {
            let line = line?;
            if let Some(op) = DB::parse_line(&line) {
                if let Op::Put { key, value } = op {
                    data.insert(key, value);
                }
            }
        }

        let writer = BufWriter::new(file);
        Ok(DB { data, writer, path: path_str })
    }

    fn parse_line(line: &str) -> Option<Op> {
        // [op:put, key:"x", val:"42"]
        let line = line.trim();
        if !line.starts_with('[') || !line.ends_with(']') { return None; }
        let inner = &line[1..line.len()-1];
        let parts: Vec<&str> = inner.split(", ").collect();

        let mut op = None;
        let mut key = None;
        let mut value = None;

        for part in parts {
            if let Some(kv) = part.strip_prefix("op:") {
                op = Some(kv.trim_matches('"'));
            } else if let Some(k) = part.strip_prefix("key:").map(|s| s.trim_matches('"')) {
                key = Some(k.to_string());
            } else if let Some(v) = part.strip_prefix("val:").map(|s| s.trim_matches('"')) {
                value = Some(v.to_string());
            }
        }

        if op == Some("put") && key.is_some() && value.is_some() {
            Some(Op::Put { key: key.unwrap(), value: value.unwrap() })
        } else {
            None
        }
    }

    pub fn put(&mut self, key: String, value: String) -> Result<(), std::io::Error> {
        let entry = format!(r#"[op:put, key:"{}", val:"{}"]\n"#, key, value);
        self.writer.write_all(entry.as_bytes())?;
        self.writer.flush()?;  // ← Durability (like fsync)
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

fn main() {
    let x: Option<i32> = Some(42);
    let y: Option<i32> = None;

    println!("x = {:?}", x);  // Some(42)
    println!("y = {:?}", y);  // None

    println!("x + 10 = {}", x.unwrap() + 10);
    // y.unwrap(); // ← PANIC! (try it)
}
