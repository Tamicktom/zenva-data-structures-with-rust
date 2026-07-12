//* Libraries imports
use std::collections::HashMap;

fn main() {
    let mut population = HashMap::new();

    population.insert("Tokyo", 37_400_100);
    population.insert("London", 17_400_100);
    population.insert("Dubai", 7_400_100);

    println!("{:?}", population);

    // for (city, pop) in &population {
    //     println!("Population of {} is {:?}", city, pop);
    // }

    // match population.get("Tokyo") {
    //     Some(&pop) => {
    //         println!("Population of Tokyo: {}", pop)
    //     }
    //     None => {
    //         println!("City not found.")
    //     }
    // }
    //

    // only insert if key doesn't exist
    // population.entry("Delhi").or_insert(32_000_000);
    // population.entry("Dubai").or_insert(9_000_000);

    // println!("{:?}", population);

    // Hashmap Performance
    // for better performance: String, i32 or usize as keys

    // pre-alocate space for 10 elements
    let mut scores = HashMap::with_capacity(10);

    for i in 0..5 {
        scores.insert(i, i * 10);
    }

    println!("Scores: {:?}", scores);
    println!("Capacity: {:?}", scores.capacity());
}
