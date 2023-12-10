use word_guessing;

fn main() {
    word_guessing::run().unwrap_or_else(|err| {
        panic!("{err}");
    });
}
