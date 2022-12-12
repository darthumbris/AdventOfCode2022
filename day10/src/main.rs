fn pushResult(cycle: i32, r: &mut Vec<i32>, x:i32, display: &mut [[char;40];6]) {
    if cycle == 20 {
        r.push(20 * x);
    }
    else if cycle > 20 && (cycle - 20) % 40 == 0 {
        r.push(cycle*x);
    }
    let mut pixel: char = '.';
    let mut pos = 0;
    let mut pos = 0;
    // let pos = (cycle + 1) % 40 - x;
    let mut xpos = 0;
    let mut ypos = 0;
    if (cycle % 40 == 0) {
        xpos = 39;
        ypos = cycle / 40 - 1;
        pos = 39 - x;
    }
    else {
        ypos = cycle / 40;
        xpos = cycle % 40 - 1;
        pos = cycle % 40 - x;
    }
    if pos >= 0 && pos <= 2 {
        pixel = '#';
    }
    println!("pos: {pos}");
    println!("drawing at: {},{}", ypos as usize, xpos as usize);
    println!("cycle{cycle}, x: {x}");
    if cycle < 240 {
    display[ypos as usize][xpos as usize] = pixel;
    }
}

fn p1(s: &str){
    let mut cycle = 0;
    let mut x = 1;
    let mut r: Vec<i32> = vec![];
    let mut display= [['.';40];6];
    for c in s.lines() {
        match c.len() {
            4 => {
                cycle+=1;
                pushResult(cycle, &mut r, x, &mut display);
            },
            _ => {
                for _i in 0..2 {
                    cycle += 1;
                    pushResult(cycle, &mut r, x, &mut display);
                }
                let number = c.split_at(5).1.parse::<i32>().unwrap();
                x += number;
            },
        }
    }
    let mut total = 0;
    for n in &r {
        total += n;
    }
    println!("{:?},{:?}", total,r);
    for i in 0..6 {
        for j in 0..40 {
            print!("{}", display[i][j]);
        }
        println!();
    }
}

fn main() {
    let s = std::fs::read_to_string("a").unwrap();
    p1(&s)
}
