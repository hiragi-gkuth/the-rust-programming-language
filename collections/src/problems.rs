use std::collections::HashMap;

pub fn mean_median_mode(list: &[i32]) -> (f64, i32, i32) {
    let mean: f64;
    let median: i32;
    let mode: i32;
    {
        mean = list.iter().sum::<i32>() as f64 / list.len() as f64;
    }
    {
        let mut sorted = list.to_vec();
        sorted.sort();
        median = sorted[sorted.len() / 2];
    }
    {
        let mut occurrences = HashMap::new();

        for &v in list {
            *occurrences.entry(v).or_insert(0) += 1;
        }

        mode = occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("Cannot compute the mode of ero numbers");
    }
    return (mean, median, mode);
}

pub fn pig_latin(original: &str) -> String {
    let vowels = "aiueo";
    let first_letter = original.chars().next().unwrap();

    if let Some(_) = vowels.find(first_letter) {
        format!("{}-hay", original)
    } else {
        let s = String::from(original);
        format!("{}-{}ay",
            s.chars().nth(1).expect("some"),
            s.chars().next().unwrap(),
        )
    }
}
