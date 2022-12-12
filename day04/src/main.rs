fn p1(s: &str)->u32{
    let mut r = 0;
    for c in s.lines() {
        let a = c.split("-");
        let mut b: [u32;4] = [0,0,0,0];
        let mut e = 0;
        for mut d in a {
            if d.contains(",") {
                for f in d.split(",") {
                    b[e] = f.parse::<u32>().unwrap();
                    e+=1;
                }
            }
            if !d.contains(",") {
                if e > 3 {
                    e = 3;
                }
                b[e] = d.trim().parse::<u32>().unwrap();
            }
            e+=1;
        }
        if (b[0] <= b[2] && b[1] >= b[3]) || (b[2] <= b[0] && b[3] >= b[1]) {
            r+=1;
        } 
    }
    r
}
fn p2(s: &str)->u32{
    let mut r = 0;
    for c in s.lines() {
        let a = c.split("-");
        let mut b: [u32;4] = [0,0,0,0];
        let mut e = 0;
        for mut d in a {
            if d.contains(",") {
                for f in d.split(",") {
                    b[e] = f.parse::<u32>().unwrap();
                    e+=1;
                }
            }
            if !d.contains(",") {
                if e > 3 {
                    e = 3;
                }
                b[e] = d.trim().parse::<u32>().unwrap();
            }
            e+=1;
        }
        if (b[0] >= b[2] && b[0] <= b[3]) || (b[1] >= b[2] && b[1] <= b[3]) ||(b[0] <= b[2] && b[1] >= b[3]) || (b[2] <= b[0] && b[3] >= b[1]) {
            println!("{:?}",b);
            r+=1;
        } 
    }
    r
}
fn main() {
    let s = std::fs::read_to_string("a").unwrap();
    print!("{},{}",p1(&s),p2(&s))
}
