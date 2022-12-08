fn main() {
    let mut y: Vec<i32> = std::fs::read_to_string("a").unwrap().split("\n\n").map(|g| g.lines().map(|x|x.parse::<i32>().unwrap()).sum()).collect();y.sort_by_key(|y|-y);
    print!("{},{}", y[0], y[0]+y[1]+y[2]);
}