use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for index in 0..a.len().min(b.len()) {
                    let order = a[index].cmp(&b[index]);
                    match order {
                        Ordering::Equal => (),
                        _ => return order,
                    }
                }

                a.len().cmp(&b.len())
            }
            (Packet::Int(a), Packet::List(_)) => Packet::List(vec![Packet::Int(*a)]).cmp(other),
            (Packet::List(_), Packet::Int(b)) => self.cmp(&Packet::List(vec![Packet::Int(*b)])),
        }
    }
}

fn parse_packet(data: &str) -> (Packet, usize) {
    let num_str = data
        .chars()
        .take_while(|c| if let '0'..='9' = c { true } else { false })
        .collect::<String>();
    if num_str.len() > 0 {
        return (Packet::Int(num_str.parse::<i32>().unwrap()), num_str.len());
    }

    let mut index = 1; // skip'['
    let mut items = Vec::new();
    while index < data.len() {
        match data.char_indices().nth(index).unwrap().1 {
            '[' | '0'..='9' => {
                // process inner value in list
                let (value, size) = parse_packet(&data[index..]);
                items.push(value);
                index += size;
            }
            ']' => break, // done list
            ',' => index += 1,
            _ => print!("err"),
        };
    }

    (Packet::List(items), index + 1)
}

fn p(s: &str){
    let pairs = s.trim_end().split("\n\n");
    let mut packets: Vec<(Packet,usize)> = vec![];

    let mut sum = 0;
    for (j,l) in pairs.enumerate() {
        let pair = l.split_once("\n").unwrap();

        let pair_1 = parse_packet(&pair.0);
        let pair_2 = parse_packet(&pair.1);
        if let Ordering::Less = pair_1.cmp(&pair_2) {
            sum += j + 1;
            packets.push(pair_1);
            packets.push(pair_2);
        }
        else {
            packets.push(pair_2);
            packets.push(pair_1);
        }
        // println!("pair1: {:?}, pair2: {:?}", pair_1.0, pair_2.0);
    }
    // println!("before sort: {:?}", packets);
    let devider_1 = parse_packet("[[2]]");
    let devider_2 = parse_packet("[[6]]");
    packets.sort();
    let mut index_1 = 1;
    for packet in packets.clone() {
        if let Ordering::Less = devider_1.cmp(&packet) {
            break;
        }
        index_1+=1;
    }
    let mut index_2 = 1;
    for packet in packets.clone() {
        if let Ordering::Less = devider_2.cmp(&packet) {
            index_2+=1;
            break;
        }
        index_2+=1;
    }
    // println!("after sort: {:?}", packets);
    println!("sum: {sum}");
    println!("p2: {},{},{}", index_1 , index_2,index_1 * index_2);

}
fn main(){
    let s=std::fs::read_to_string("a").unwrap();
    p(&s);
}