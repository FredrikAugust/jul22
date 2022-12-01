use std::{collections::HashMap, fs::File, io::Read};

/*
 * Prøvde først en klassisk brute-force i håp om at de hadde vært snille og gitt en løsning uten overlappende ord.
 * Det hadde de naturligvis ikke så da ble det denne løsningen. Løs for ord, og sjekk om resterende streng er gyldig.
 * Dersom den ikke er det tar vi et lengre ord.
 */

fn solve_recursively(dictionary: &HashMap<String, String>, input: String) -> Option<Vec<String>> {
    let mut buffer = String::new();
    let mut input_clone = input.clone();

    if input_clone.is_empty() {
        return Some(vec![]);
    }

    loop {
        if input_clone.is_empty() {
            return None;
        }

        buffer += &input_clone[0..1];
        input_clone = input_clone[1..].to_string();

        // Løs for ord
        let Some(value) = dictionary.get(buffer.as_str()) else {
            continue;
        };

        // Løs for resterende ord
        let Some(rest_solution) = solve_recursively(dictionary, input_clone.clone()) else {
            continue;
        };

        let mut new_result = vec![value.to_string()];
        new_result.extend(rest_solution);

        return Some(new_result);
    }
}

fn main() {
    let mut dictionary_string = String::new();

    File::open("src/dictionary.txt")
        .unwrap()
        .read_to_string(&mut dictionary_string)
        .unwrap();

    let mut dictionary: HashMap<String, String> = HashMap::new();

    dictionary_string.lines().for_each(|line| {
        let mut split = line.split(",");
        let key = split.next().unwrap();
        let value = split.next().unwrap();

        dictionary.insert(key.to_string(), value.to_string());
    });

    let mut letter_string = String::new();

    File::open("src/letter.txt")
        .unwrap()
        .read_to_string(&mut letter_string)
        .unwrap();

    let solution = solve_recursively(&dictionary, letter_string)
        .expect("Vi kan da kode for svingende. Dette feiler ikke.");

    println!("Løsningen er: {}.", solution.join(" ").len());
}
