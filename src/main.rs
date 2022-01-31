use std::fs;
mod lib;

fn main() {
    const OUTPUT_LENGTH: usize = 1000;
    // fetch and process input:
    let contents = fs::read_to_string("content.txt").expect("Something went wrong");
    let split: Vec<String> = lib::process_input(contents);

    // Record words in an IndexMap with their count (dont need count atm but might later)
    let string_values = lib::record_words(&split);

    // Record stochastic matrix of Markov chain:
    let length = string_values.len();
    let markov_array = lib::get_markov_array(length, &split, &string_values);

    // traverse the chain at random, starting at START_POS
    lib::generate_output(&length, OUTPUT_LENGTH, &string_values, &markov_array);
}
