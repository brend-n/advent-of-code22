fn main() {
    println!(
        "{}",
        include_str!("../inp")
        .split("\n\n")
        .map(|d| d.lines().map(|f| f.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap(),
    );
}