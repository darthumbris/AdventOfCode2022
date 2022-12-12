use std::collections::HashSet;
fn vert(h:[i32;2],k:&mut[i32;2]){
    if h[0]>k[0]{for i in k[0]..h[0]{k[0]=i;}}
    else{for i in((h[0]+1)..k[0]).rev(){k[0]=i;}}
}

fn hor(h:[i32;2],k:&mut[i32;2]) {
    if h[1]>k[1]{for i in k[1]..h[1]{k[1]=i;}}
    else{for i in((h[1]+1)..k[1]).rev(){k[1]=i;}}
}

fn diag(h:[i32;2],k:&mut[i32;2]){
    if h[1]>k[1]{
        k[1]+=1;
        if h[0]>k[0]{k[0]+=1;}
        else{k[0]-=1;}
    }
    else {
        k[1]-=1;
        if h[0]>k[0]{k[0]+=1;}
        else{k[0]-=1;}
    }
    if h[0]!=k[0]{vert(h, k);}
    else if h[1]!=k[1]{hor(h, k);}
}

fn mov(k:&mut[[i32;2];10]){
    for i in 1..10{
        if (k[i][0]-k[i-1][0]).abs()>1&&k[i][1]==k[i-1][1]{vert(k[i-1],&mut k[i]);}
        if k[i][0]==k[i-1][0]&&(k[i][1]-k[i-1][1]).abs()>1 {hor(k[i-1],&mut k[i]);}
        if (k[i][0]-k[i-1][0]).abs()>1||(k[i][1]-k[i-1][1]).abs()>1{diag(k[i-1], &mut k[i]);}
    }
}

fn p(s:&str){
    let mut p=[HashSet::new(),HashSet::new()];
    let mut k=[[4,0];10];
    for l in s.lines(){
        let c=l.split_once(" ").unwrap();
        let mut s=[0,0];
        let r:i32=c.1.parse().unwrap();
        match c.0.as_bytes()[0]{76=>s[1]-=r,82=>s[1]+=r,68=>s[0]+=r,85=>s[0]-=r,_ => println!("other")}
        let mut d=[0,0];
        let m:i32 = if s[0].abs()>0{s[0].abs()}else{s[1].abs()};
        if s[0]>0{d[0]=1;}
        if s[0]<0{d[0]=-1;}
        if s[1]>0{d[1]=1;}
        if s[1]<0{d[1]=-1;}
        for _i in 0..m {
            k[0][0]+=d[0];
            k[0][1]+=d[1];
            mov(&mut k);
            p[1].insert(k[9]);
            p[0].insert(k[1]);
        }
    }
    println!("{},{}",p[0].len(), p[1].len());
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    p(&s)
}