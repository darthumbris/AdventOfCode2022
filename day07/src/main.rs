use std::path::PathBuf;
use std::collections::HashMap;

fn p(s:&str){
    let mut c: PathBuf = PathBuf::new();
    let mut r:HashMap<PathBuf, i32> = HashMap::new();
    for line in s.lines() {
        let mut b = line.split(" ");
        let u = b.nth(0).unwrap();
        if line.starts_with("$ cd") {
            let dir = b.nth(1).unwrap();
            if dir == ".." {c.pop();}
            else {c.push(dir);}   
        }
        if !u.contains("dir") && !u.contains("$") {
            let o = u.parse::<i32>().unwrap();
            for anc in c.ancestors() {
                if anc != c {
                    let mut parent: PathBuf = PathBuf::new();
                    parent.push(anc);
                    *r.entry(parent.clone()).or_insert(0) += o;
                }
            }
            *r.entry(c.clone()).or_insert(0) += o;
        }
    }
    let mut diff=0;
    let mut p1 = 0;
    for u in r.clone() {
        if u.0.as_path().to_str().unwrap() == "/" {
            diff = 70000000 - u.1;
        }
        if u.1 < 100000 {
            p1 += u.1;
        }
    }
    let mut dir_maybe_del: Vec<i32> = vec![];
    let size_needed = 30000000- diff;
    for u in r.clone() {
        if u.1 > size_needed {
            dir_maybe_del.push(u.1);
        }
    }
    dir_maybe_del.sort();
    let p2 = dir_maybe_del.first().unwrap();
    println!("p1: {}\np2:{}",p1, p2);
    println!("{size_needed}");
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    p(&s);
}