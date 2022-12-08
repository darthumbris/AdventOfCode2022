fn p(s:&str,e:[i32;6])->i32{
    s.lines().map(|l|match l{
    "A X"=>e[0],"A Y"=>e[1],"A Z"=>e[2],
    "B X"=>1,"B Y"=>5,"B Z"=>9,
    "C X"=>e[3],"C Y"=>e[4],"C Z"=>e[5],
    _=>0}).sum()
}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    print!("{},{}",p(&s,[4,8,3,7,2,6]),p(&s,[3,4,8,2,6,7]))
}