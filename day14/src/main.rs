use std::collections::HashSet;
fn p1(c:&mut HashSet<(usize,usize)>,f:usize)->usize{
    let mut p:(usize,usize)=(500,0);
    let mut s=0;
    while p.1<f{
        if c.contains(&(p.0,p.1+1)){
            if c.contains(&(p.0-1,p.1+1)){
                if c.contains(&(p.0+1,p.1+1)){c.insert(p);s+=1;p=(500,0);}
                else{p.0+=1;p.1+=1;}
            }
            else{p.0-=1;p.1+=1;}
        }
        else{p.1+=1}
    }
    s
}

fn drop_sand(c:&mut HashSet<(usize, usize)>,s:(usize,usize),f:usize){
    if c.contains(&s)||s.1==f{return}
    drop_sand(c,(s.0,s.1+1),f);
    drop_sand(c,(s.0-1,s.1+1),f);
    drop_sand(c,(s.0+1,s.1+1),f);
    c.insert(s);
}

fn p2(c:&mut HashSet<(usize, usize)>,f:usize)->usize{
    let obj=c.len();
    drop_sand(c,(500,0),f);
    c.len()-obj
}

fn p(s: &str)->(usize,usize){
    let mut c:HashSet<(usize, usize)>=HashSet::new();
    for l in s.lines(){
        let cords=l.split(" -> ");
        let mut points:Vec<(usize,usize)>=vec![];
        for p in cords{
            let q = p.split_once(",").unwrap();
            let pos=(q.0.parse::<usize>().unwrap(),q.1.parse::<usize>().unwrap());
            points.push(pos);
        }
        for i in 0..points.len() - 1 {
            let mut p = points[i];
            while p.0!=points[i+1].0{
                c.insert(p);
                if p.0<points[i+1].0{p.0 += 1;}
                else{p.0 -=1;}
            }
            while p.1!=points[i+1].1{
                c.insert(p);
                if p.1<points[i+1].1{p.1 += 1;}
                else{p.1 -=1;}
            }
        }
        c.insert(*points.last().unwrap());
    }
    let mut f = 0;
    for r in c.clone(){if r.1>f{f=r.1;}}
    return (p1(&mut c.clone(),f),p2(&mut c.clone(),f+2))
}

fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    println!("{:?}",p(&s));
}