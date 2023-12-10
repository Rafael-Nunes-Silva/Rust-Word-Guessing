use word_guessing;

fn main() {
    println!("Hello, world!");

    word_guessing::run().unwrap_or_else(|err| {
        panic!("{err}");
    });
}
