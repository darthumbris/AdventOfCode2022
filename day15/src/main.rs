
use std::{collections::BTreeSet, collections::BTreeMap};
fn q(s: &str)->i64{
    let mut rays:BTreeMap::<(i64,i64),BTreeSet::<i64>>=BTreeMap::new();
    for l in s.lines(){
        let pos=l.split(",");
        let mut sb=[0;4];
        for (i,p)in pos.enumerate(){sb[i] = p.parse::<i64>().unwrap();}
        let manh_dist = (sb[0].abs_diff(sb[2]) + sb[1].abs_diff(sb[3]) + 1) as i64; // + 1 so that I can check where they intersect
        let dir = [(-1, -1), (-1, 1), (1, -1), (1, 1)]; // the four kinds of dir for the rays (down-left, down-right, up-left and up-right)
        for (m,n) in  dir{
            let b = sb[1] + n * manh_dist - m * sb[0];
            if rays.contains_key(&(m,b)) { //ray already has this line
                let mut set:BTreeSet::<i64> = rays.get(&(m,b)).unwrap().to_owned();
                set.insert(n);
                rays.insert((m,b), set);
            }
            else {
                let mut set = BTreeSet::new();
                set.insert(n);
                rays.insert((m,b),set);
            }
        }
    }
    let mut lines: Vec<(i64, i64)> = vec![];
    for ray in rays{if ray.1.len()==2{lines.push(ray.0)}}//if there are 2 rays with a set len of 2 that means they are intersecting there
    let m1=lines[0].0;
    let b1=lines[0].1;
    let m2=lines[1].0;
    let b2=lines[1].1;
    let x=(b2-b1)/(m1-m2);
    let y=m1*x+b1;
    x*4000000+y
}

fn p(s: &str)->i32{
    let to_reach = 2000000;
    let mut coverage:BTreeSet::<(i32, i32)> = BTreeSet::new();
    let mut beacons: BTreeSet::<(i32, i32)> = BTreeSet::new();
    for l in s.lines() {
        let pos = l.split(",");
        let mut sb = [0;4];
        for (i,p)in pos.enumerate(){sb[i] = p.parse::<i32>().unwrap();}
        beacons.insert((sb[2],sb[3]));
        let manh_dist = (sb[0].abs_diff(sb[2]) + sb[1].abs_diff(sb[3])) as i32;
        if sb[1].abs_diff(to_reach) > manh_dist as u32 {
            continue;
        }
        
        for j in sb[0]-manh_dist..sb[0]+manh_dist+1 {
            if to_reach.abs_diff(sb[1]) + j.abs_diff(sb[0]) <= manh_dist as u32{
                coverage.insert((j, to_reach));
            }
        }
    }
    for beacon in beacons {
        if coverage.contains(&beacon) {
            coverage.remove(&beacon);
        }
    }
    let mut r = 0;
    for coord in coverage {
        if coord.1 == to_reach {
            r+=1;
        }
    }
    r
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    let mut r = s.replace("Sensor at x=", "");
    r = r.replace(", y=", ",");
    r = r.replace(": closest beacon is at x=", ",");
    println!("{},{}",p(&r),q(&r))
}