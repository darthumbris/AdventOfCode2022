fn p1(s: &str)->u32{
    let mut r = 0;
    for c in s.lines() {
        let a = c.split_at(c.len() / 2).0;
        let b = c.split_at(c.len() / 2).1;
        for d in a.chars() {
            if b.contains(d) {
                if d.is_ascii_lowercase() {
                    
                    r += d as u32 - 96;
                }
                else {
                    r+= d as u32 - 38;
                }
                break;
            }
        }
    }
    r
}
fn p2(s:&str)->u32{
    let mut r = 0;
    let m: Vec<&str> = s.lines().collect();
    let mut l = 0;
    loop {
        let a = m[l];
        let b = m[l + 1];
        let c = m[l + 2];

        for d in a.chars() {
            if b.contains(d) && c.contains(d) {
                println!("{}",d);
                if d.is_ascii_lowercase() {
                    r += d as u32 - 96;
                }
                else {
                    r+= d as u32 - 38;
                }
                break;
            }
        }
        l+=3;
        if l > m.len() - 3 {
            break;
        }
    }
    r
}
fn main() {
    let s = std::fs::read_to_string("a").unwrap();
    print!("{},{}",p1(&s),p2(&s))
}
