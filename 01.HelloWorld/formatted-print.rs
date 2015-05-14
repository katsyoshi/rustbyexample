fn main() {
    println!("{} days", 31);

    println!("{subject} {verb} {predicate}",
             predicate = "over lazy dog",
             subject = "the quick brown fox",
             verb = "jump");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("My name is {0}, {1} {0}", "Bond");

    struct Structure(i32);
    println!("The struct `{}` won't print...", Structure(3));
}
