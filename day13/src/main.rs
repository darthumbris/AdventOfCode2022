use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet{List(Vec<Packet>),Int(i32)}
impl PartialOrd for Packet{
    fn partial_cmp(&self,other:&Self)->Option<Ordering>{Some(self.cmp(other))}
}
impl Ord for Packet{
    fn cmp(&self,other:&Self)->Ordering{
        match (self,other){
            (Packet::Int(a),Packet::Int(b))=>a.cmp(b), //compare int to int
            (Packet::List(a),Packet::List(b))=>{ //compare list to list
                for i in 0..a.len().min(b.len()){
                    let ord=a[i].cmp(&b[i]);
                    match ord {Ordering::Equal=>(),_=>return ord}
                }
                a.len().cmp(&b.len())
            }
            (Packet::Int(a),Packet::List(_))=>Packet::List(vec![Packet::Int(*a)]).cmp(other), //compare int to empty list
            (Packet::List(_),Packet::Int(b))=>self.cmp(&Packet::List(vec![Packet::Int(*b)])) //compare empty list to int
        }
    }
}

fn pp(d:&str)->(Packet,usize){
    let n=d.chars().take_while(|c|if let '0'..='9'=c{true}else{false}).collect::<String>();
    if n.len()>0{return(Packet::Int(n.parse::<i32>().unwrap()),n.len())}
    let mut i=1;//skip first [
    let mut r=Vec::new();
    while i<d.len(){
        match d.char_indices().nth(i).unwrap().1{
            '['|'0'..='9'=>{let(x,l)=pp(&d[i..]);r.push(x);i+=l}// process inner value in list
            ']'=>break,//end list
            ','=>i+=1,//next item
            _=>print!("")
        };
    }
    (Packet::List(r),i+1)
}
fn p(s: &str){
    let pairs=s.trim_end().split("\n\n");
    let mut list=vec![];
    let mut r=0;
    for(j,l)in pairs.enumerate(){
        let p=l.split_once("\n").unwrap();
        let p1=pp(&p.0);
        let p2=pp(&p.1);
        if let Ordering::Less=p1.cmp(&p2){r+=j+1;list.push(p1);list.push(p2)}
        else{list.push(p2);list.push(p1)}
    }
    let d1=pp("[[2]]");
    let d2=pp("[[6]]");
    list.sort();
    let mut i=1;
    for p in list.clone(){
        if let Ordering::Less=d1.cmp(&p){break}
        i+=1;
    }
    let mut j=1;
    for p in list{
        if let Ordering::Less=d2.cmp(&p){println!("{r},{}",i*(j+1));break}
        j+=1;
    }
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    p(&s)
}