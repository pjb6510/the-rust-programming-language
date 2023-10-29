#![allow(unused)]

use std::collections::HashMap;

fn main() {
    access_hash_map();
    update_hash_map();
}

fn create_hash_map() {
    let mut scores = HashMap::new();

    scores.insert("blue".to_string(), 10);
    scores.insert("red".to_string(), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

fn show_hash_map_ownership() {
    let field_name = "Favorite color".to_string();
    let field_value = "Blue".to_string();

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // error!
    // println!("field_name: {}", field_name);
}

fn access_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_names = vec!["Red".to_string(), "Blue".to_string()];

    println!("\nscores by team!");
    for team_name in &team_names {
        let score = scores.get(team_name);

        match score {
            Some(&score) => println!("score is {}", score),
            None => (println!("team_name \"{team_name}\" is unvalid")),
        }
    }

    println!("\nscore!");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn update_hash_map() {
    println!("\nupdate_hash_map");

    fn overwrite_hash_map() {
        println!("\noverwrite_hash_map");
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    fn use_entry() {
        println!("\nuse_entry");
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);

        scores.entry("Yellow".to_string()).or_insert(50);
        scores.entry("Blue".to_string()).or_insert(50);

        println!("{:?}", scores)
    }

    fn use_prev_key() {
        println!("\nuse_prev_key");
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }

    overwrite_hash_map();
    use_entry();
    use_prev_key();
}
