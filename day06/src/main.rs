fn p(s:&str,l:usize)->usize{
    let mut r=l;
    let mut v=std::collections::HashSet::new();
    for i in s.as_bytes().windows(l){
        for u in i{v.insert(u);}
        if v.len()==l{break}
        v.clear();
        r+=1;
    }
    r
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    print!("{},{}",p(&s,4),p(&s,14))
}