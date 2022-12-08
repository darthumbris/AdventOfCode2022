fn p(s:&str){
    let mut grid:Vec<Vec<char>>=vec![vec![];99];
    for (r,l) in s.lines().enumerate(){for t in l.chars() {grid[r].push(t);}}
    let mut vis_score: Vec<i32> = vec![];
    for i in 1..grid.len()-1{
        for j in 1..grid[i].len()-1{
            let t=grid[i][j];
            let mut s=[0,0,0,0,4];
            for l in(0..j).rev(){
                s[0]+=1;
                if grid[i][l]>=t{s[4]-=1;break}
            }
            for r in j+1..grid[i].len(){
                s[1]+=1;
                if grid[i][r]>=t{s[4]-=1;break}
            }
            for u in(0..i).rev() {
                s[2]+=1;
                if grid[u][j]>=t{s[4]-=1;break}
            }
            for d in i+1..grid.len(){
                s[3]+=1;
                if grid[d][j]>=t{s[4]-=1;break}
            }
            if s[4]>0{vis_score.push(s[0]*s[1]*s[2]*s[3]);}
        }
    }
    vis_score.sort();
    println!("{},{:?}",vis_score.len() + grid.len()*2+(grid[0].len()-2)*2, vis_score.last().unwrap());
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    p(&s);
}