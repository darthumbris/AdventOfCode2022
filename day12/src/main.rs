use std::collections::VecDeque;

const WIDTH: usize  = 172;
const HEIGHT: usize = 41;

const D_ROW: [i32;4] = [ -1, 0, 1, 0 ];
const D_COL: [i32;4] = [ 0, 1, 0, -1 ];

fn ok(to:(i32,i32),fr:(i32,i32),v:[[bool;WIDTH];HEIGHT],g:[[char;WIDTH];HEIGHT],p:bool)->bool{
    if to.0<0||to.1<0||to.0>=HEIGHT as i32||to.1>=WIDTH as i32||!w(g,to,fr,p){return false}
    if v[to.0 as usize][to.1 as usize]{return  false}
    true
}

fn w(g: [[char;WIDTH];HEIGHT], to: (i32, i32), fr: (i32, i32), p: bool)->bool{
    let d=(g[to.0 as usize][to.1 as usize]as i32-g[fr.0 as usize][fr.1 as usize]as i32)as i32;
    if d<=1&&p||d>=-1&&!p{return true}
    false
}

fn bfs(src:(i32, i32),dest:(i32,i32),g:[[char;WIDTH];HEIGHT],p:bool)->i32{
    let mut v=[[false;WIDTH];HEIGHT];
    let mut q=VecDeque::new();
    v[src.0 as usize][src.1 as usize]=true;
    q.push_back((src.0, src.1, 0));
    let mut r=0;
    while !q.is_empty() {
        let c=q.front().unwrap();
        let y=c.0;
        let x=c.1;
        r=c.2;
        if x==dest.1 && y==dest.0 || (g[y as usize][x as usize]=='a'&&!p){break}
        q.pop_front();
        for k in 0..4{
            let j=x+D_COL[k];
            let i=y+D_ROW[k];
            if ok((i,j),(y,x),v,g,p){
                q.push_back((i,j,r+1));
                v[i as usize][j as usize] = true;
            }
        }
    }
    r
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    let mut grid=[['0';WIDTH];HEIGHT];
    let mut src=(0,0);
    let mut dest=(0,0);
    for(i,l)in s.lines().enumerate(){
        for(j,c)in l.chars().enumerate() {
            match c{
                'S'=>{grid[i][j]='a';src=(i as i32,j as i32);},
                'E'=>{grid[i][j]='z'; dest=(i as i32,j as i32);},
                _=>grid[i][j] = c,
            }
        }
    }
    println!("{},{}", bfs(src, dest, grid, true), bfs(dest, src, grid, false));
}