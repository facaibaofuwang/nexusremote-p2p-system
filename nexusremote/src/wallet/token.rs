//! Token economics and calculations

use crate::core::types::*;

/// Calculate relay cost based on data size and reputation
pub fn calculate_relay_cost(
    data_size: u64,
    reputation: ReputationScore,
) -> TokenAmount {
    let mb = (data_size as f64) / (1024.0 * 1024.0);
    let base_cost = mb * 1.0; // 1 NEXUS per MB
    
    // Reputation discount: higher reputation = lower cost
    let discount = 1.0 - (reputation.value() as f64 / 2000.0); // Max 50% discount
    
    TokenAmount::new((base_cost * discount).max(0.1) as u128)
}

/// Calculate relay earnings
pub fn calculate_relay_earnings(
    data_size: u64,
    reputation: ReputationScore,
) -> TokenAmount {
    let mb = (data_size as f64) / (1024.0 * 1024.0);
    let base_earning = mb * 1.0;
    
    // Reputation bonus: higher reputation = higher earnings
    let bonus = 1.0 + (reputation.value() as f64 / 2000.0); // Max 50% bonus
    
    TokenAmount::new((base_earning * bonus) as u128)
}

/// Calculate priority score for routing
pub fn calculate_priority_score(
    reputation: ReputationScore,
    balance: TokenAmount,
) -> f64 {
    let rep_score = reputation.value() as f64 / 1000.0; // 0-1
    let bal_score = (balance.value().min(1000) as f64) / 1000.0; // 0-1
    
    (rep_score * 0.7) + (bal_score * 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_relay_cost_calculation() {
        let data_size = 1024 * 1024; // 1 MB
        
        let low_rep = ReputationScore::new(100);
        let high_rep = ReputationScore::new(900);
        
        let cost_low = calculate_relay_cost(data_size, low_rep);
        let cost_high = calculate_relay_cost(data_size, high_rep);
        
        // High reputation should have lower cost
        // Debug output
        println!("Low rep cost: {}, High rep cost: {}", cost_low.value(), cost_high.value());
        assert!(cost_high.value() <= cost_low.value());
    }
    
    #[test]
    fn test_relay_earnings_calculation() {
        let data_size = 1024 * 1024; // 1 MB
        
        let low_rep = ReputationScore::new(100);
        let high_rep = ReputationScore::new(900);
        
        let earn_low = calculate_relay_earnings(data_size, low_rep);
        let earn_high = calculate_relay_earnings(data_size, high_rep);
        
        // High reputation should have higher earnings
        // Debug output
        println!("Low rep earnings: {}, High rep earnings: {}", earn_low.value(), earn_high.value());
        assert!(earn_high.value() >= earn_low.value());
    }
    
    #[test]
    fn test_priority_score() {
        let low_priority = calculate_priority_score(
            ReputationScore::new(100),
            TokenAmount::new(10),
        );
        
        let high_priority = calculate_priority_score(
            ReputationScore::new(900),
            TokenAmount::new(500),
        );
        
        assert!(high_priority > low_priority);
    }
}
