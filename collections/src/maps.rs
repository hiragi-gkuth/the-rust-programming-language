use std::collections::HashMap;

pub fn use_of_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Blue")).or_insert(60);

    let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Red")];
    let initial_scores = vec![10, 50, 30];

    let team_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
    println!("{:?}", team_scores);
    
    let score = scores.get("Blue");
    if let Some(s) = score {
        print!("{}", s);
    }
}

pub fn word_counter() {
    let text = "This document defines the core of the QUIC transport protocol. QUIC provides applications with flow-controlled streams for structured communication, low-latency connection establishment, and network path migration. QUIC includes security measures that ensure confidentiality, integrity, and availability in a range of deployment circumstances. Accompanying documents describe the integration of TLS for key negotiation, loss detection, and an exemplary congestion control algorithm.";
    
    let mut words = HashMap::new();
    
    for word in text.split_ascii_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", words);
}
