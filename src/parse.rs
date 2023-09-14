use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

pub fn parse() -> (String, Vec<Vec<String>>) {
    let file = OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open("server.conf")
        .expect("Cant open server.conf");
    let mut line_iter = BufReader::new(file).lines();
    let mut proxy = line_iter
        .next()
        .unwrap()
        .expect("Need a proxy port, eg: 8080:");
    let _ = proxy.trim();
    proxy.pop();
    let mut vec = Vec::new();
    let mut collision_detec = HashSet::new();
    for line in line_iter {
        let line = line.unwrap();
        let mut v = Vec::new();
        if line.contains("+") {
            let parts: Vec<_> = line.split("+").collect();
            let start = parts[0].trim().parse::<i32>().unwrap();
            let len = parts[1].trim().parse::<i32>().unwrap() + 1;
            for i in 0..len {
                v.push((start + i).to_string());
                if !collision_detec.insert(start + i) {
                    panic!("duplicated port {}", start + i)
                }
            }
            vec.push(v);
        }
    }
    (proxy, vec)
}
