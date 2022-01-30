use indexmap::IndexMap;
use ndarray::Array2;
use rand::Rng;
use std::fs;

fn main() {
    let contents = fs::read_to_string("content.txt").expect("Something went wrong");

    // println!("\n{}", contents);

    let split: Vec<String> = contents
        .trim()
        .split(" ")
        .map(|s| {
            s.to_string()
                .replace("\"", "")
                .replace(",", "")
                .replace("“", "")
        })
        .collect();

    let mut string_values = IndexMap::new();

    for word in &split {
        if string_values.contains_key(&word) {
            *string_values.get_mut(&word).unwrap() += 1;
        } else {
            string_values.insert(word, 1);
        }
    }

    // for (key, value) in &string_values {
    //     println!("{} {}", key, value);
    // }

    let length = string_values.len();

    let mut markov_array = Array2::<f64>::zeros((length, length));

    // loop through split, check next word, find index of that word and add 1 to that element
    // in markov array

    for (i, _word) in (&split).iter().enumerate() {
        if i + 1 < split.len() {
            let column = string_values.get_index_of(&split[i]).unwrap();
            let row = string_values.get_index_of(&split[i + 1]).unwrap();
            markov_array[[row, column]] += 1_f64;
        }
    }

    for j in 0..length {
        let mut sum = 0_f64;
        for i in 0..length {
            sum += &markov_array[[i, j]];
        }
        if sum > 0_f64 {
            for i in 0..length {
                markov_array[[i, j]] = markov_array[[i, j]] / sum;
            }
        }
    }
    // println!("{}", markov_array);
    let mut start = 8_usize;
    let value = string_values.get_index(start).unwrap();
    let mut results: Vec<&str> = vec![];
    results.push(value.0);
    for i in 0..600 {
        start = traverse_chain(start, &markov_array, length);
        let x = string_values.get_index(start);
        if x == None {
            break;
        }
        let value = string_values.get_index(start).unwrap();
        results.push(value.0);
    }
    // println!("{}", results.join(" "));
    let result = results.join(" ");
    let _end = fs::write("result.txt", result);
}

fn traverse_chain(start: usize, markov_array: &Array2<f64>, length: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random = rng.gen::<f64>();
    let mut current_size = 0_f64;
    let mut j: usize = 0;
    for i in 0..length {
        current_size += markov_array[[i, start]];
        if random < current_size {
            break;
        }
        j += 1;
    }
    j
}
