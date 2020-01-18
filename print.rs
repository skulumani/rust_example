fn main() {
    // replaces {} kind of like python
    println!("{} days", 31);

    // can do positional printing 
    println!("{0}, this is {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}", 
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

}
