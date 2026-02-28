use nexusremote::core::types::ReputationScore;

fn main() {
    let min_rep = ReputationScore::MIN;
    let max_rep = ReputationScore::MAX;
    
    println!("MIN reputation: {}", min_rep.value());
    println!("MAX reputation: {}", max_rep.value());
    
    // Calculate factors
    let min_factor = 2000.0 / (min_rep.value() as f64 + 1000.0);
    let max_factor = 2000.0 / (max_rep.value() as f64 + 1000.0);
    
    println!("MIN factor: {}", min_factor);
    println!("MAX factor: {}", max_factor);
}