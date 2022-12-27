fn main() {
    let mut dwarves = 
        include_str!("../inp")
        .split("\n\n")
        .map(|d| d.lines().map(|f| f.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    dwarves.sort_unstable_by(|a, b| b.cmp(a));
    println!("{}", dwarves.into_iter().take(3).sum::<u32>());
}
