use indexmap::IndexMap;
use ndarray::Array2;
use rand::Rng;
use std::fs;

pub fn process_input(input: String) -> Vec<String> {
    input
        .trim()
        .split(" ")
        .map(|s| {
            s.to_string()
                .replace("\"", "")
                .replace(",", "")
                .replace("“", "")
                .replace("”", "")
        })
        .collect()
}

pub fn record_words(split: &Vec<String>) -> IndexMap<&String, u32> {
    let mut string_values = IndexMap::new();
    for word in split {
        if string_values.contains_key(&word) {
            *string_values.get_mut(&word).unwrap() += 1_u32;
        } else {
            string_values.insert(word, 1_u32);
        }
    }
    string_values
}

pub fn generate_output(
    length: &usize,
    output_length: usize,
    string_values: &IndexMap<&String, u32>,
    markov_array: &Array2<f64>,
) {
    let value = string_values.get_index(0).unwrap();
    let mut results: Vec<&str> = vec![];
    results.push(value.0);
    let mut start = 0_usize;
    for _i in 0..output_length {
        start = traverse_chain(start, &markov_array, *length);
        let x = string_values.get_index(start);
        if x == None {
            break;
        }
        let value = string_values.get_index(start).unwrap();
        results.push(value.0);
    }
    // println!("{}", results.join(" "));
    let result = results.join(" ").replace('\n', " ");
    let _end = fs::write("result.txt", result);
}

pub fn traverse_chain(start: usize, markov_array: &Array2<f64>, length: usize) -> usize {
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

pub fn get_markov_array(
    length: usize,
    split: &Vec<String>,
    string_values: &IndexMap<&String, u32>,
) -> Array2<f64> {
    let mut markov_array = Array2::<f64>::zeros((length, length));

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
    markov_array
}
