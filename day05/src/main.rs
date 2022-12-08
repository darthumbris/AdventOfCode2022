fn p(s:&str){
    let mut l:Vec<Vec<char>>=vec![vec![];9];
    for (v,a)in s.lines().enumerate(){
        if v<8{
            for (u,b)in a.chars().skip(1).step_by(4).zip(&mut l)
            {if u>' '{b.insert(0,u);}}
        }
        if v>9{
            let e:Vec<_>=a.split(" ").skip(1).step_by(2).map(str::parse::<usize>).map(Result::unwrap).collect();
            for _f in 0..e[0]{
                let t=l[e[1]-1].pop().unwrap();
                l[e[2]-1].push(t);
            }
        }
    }
    for r in l{print!("{}",r.last().unwrap());}
}
fn q(s:&str){
    let mut l: Vec<Vec<u8>> = Vec::new();
    for _k in 0..9 {
        l.push(Vec::new());
    }
    for a in s.lines() {
        if a.len() == 35 && a.contains("[") {
            for i in 0..9 {
                if a.as_bytes()[i * 4 + 1] != b' ' {
                    l[i].push(a.as_bytes()[i * 4 + 1]);
                }
            }
        }
        else if a.len() == 0 {
            for k in 0..9 {
                l[k].reverse();
            }
        }
        else if a.len() > 1 && a.len() != 35 {
            let c = &a.replace("move ", "").replace(" from ", ",").replace(" to ", ",");
            let b = c.split(",");
            let mut e:[usize;3] = [0,0,0];
            let mut i = 0;
            for d in b {
                e[i] = d.parse::<usize>().unwrap();
                i+=1;
            }
            for f in (0..e[0]).rev() {
                let temp = l[e[1]-1][l[e[1] -1].len() - (f + 1)];
                l[e[2] - 1].push(temp);
            }
            for _f in 0..e[0] {
                l[e[1]-1].pop();
            }
        }
    }
    for lol in 0..9 {
        print!("{}", *l[lol].last().unwrap() as char);
    }
}
fn main(){
    let s=std::fs::read_to_string("j").unwrap();
    p(&s);
    print!(",");
    q(&s);
}