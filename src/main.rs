use std::collections::HashMap;
use std::io::{self,Read};

fn main() {
    let mut buffer = String::new();
    let mut counter = HashMap::new();
    io::stdin().read_to_string(&mut buffer);
    for c in buffer.chars() {
        let val = counter.entry(c).or_insert(0);
        *val += 1;
    }
    let mut sorted_vec : Vec<_> = counter.iter().collect();
    sorted_vec.sort_unstable_by(|(ac,ai),(bc,bi)| bi.cmp(ai));
    for (c,i) in &sorted_vec {
        println!("{:?},{}",c,i);
    }
}
