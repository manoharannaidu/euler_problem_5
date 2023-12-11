use num::integer::lcm;

fn main() {
    let mut curr_lcm:u32 = 1;
    for n in 1..=20 {
        curr_lcm = lcm(n,curr_lcm);
    };
    print!("{curr_lcm}")
}
