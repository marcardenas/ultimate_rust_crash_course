// Part 3
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // Part 1
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    // Part 2
    missiles =  missiles - ready;

    println!("{} missiles left", missiles)
}
