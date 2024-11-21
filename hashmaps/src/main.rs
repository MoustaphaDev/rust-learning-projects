use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&String, i32> = HashMap::new();
    let barca = String::from("Barca");
    let real = String::from("Real");

    scores.insert(&barca, 4);
    scores.insert(&real, 0);

    let barca_score = scores.get(&barca).unwrap_or(&0);
    println!("Barca scored {barca_score} goals!");
}
