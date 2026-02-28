//! Statistics and analysis for simulations

use crate::core::types::*;
use std::collections::HashMap;

/// Analyze routing distribution
pub fn analyze_routing_distribution(
    distribution: &HashMap<DeviceID, usize>,
    reputations: &HashMap<DeviceID, ReputationScore>,
) -> RoutingAnalysis {
    let mut high_rep_count = 0;
    let mut low_rep_count = 0;
    let mut high_rep_selections = 0;
    let mut low_rep_selections = 0;
    
    for (device_id, &count) in distribution {
        if let Some(&reputation) = reputations.get(device_id) {
            if reputation.value() >= 700 {
                high_rep_count += 1;
                high_rep_selections += count;
            } else {
                low_rep_count += 1;
                low_rep_selections += count;
            }
        }
    }
    
    let total_selections = high_rep_selections + low_rep_selections;
    let high_rep_rate = if total_selections > 0 {
        high_rep_selections as f64 / total_selections as f64
    } else {
        0.0
    };
    
    let high_rep_population = if high_rep_count + low_rep_count > 0 {
        high_rep_count as f64 / (high_rep_count + low_rep_count) as f64
    } else {
        0.0
    };
    
    let advantage_ratio = if high_rep_population > 0.0 {
        high_rep_rate / high_rep_population
    } else {
        0.0
    };
    
    RoutingAnalysis {
        high_rep_count,
        low_rep_count,
        high_rep_selections,
        low_rep_selections,
        high_rep_rate,
        high_rep_population,
        advantage_ratio,
    }
}

/// Routing analysis results
#[derive(Debug, Clone)]
pub struct RoutingAnalysis {
    /// Number of high reputation nodes
    pub high_rep_count: usize,
    /// Number of low reputation nodes
    pub low_rep_count: usize,
    /// Total selections for high reputation nodes
    pub high_rep_selections: usize,
    /// Total selections for low reputation nodes
    pub low_rep_selections: usize,
    /// Selection rate for high reputation nodes
    pub high_rep_rate: f64,
    /// Population percentage of high reputation nodes
    pub high_rep_population: f64,
    /// Advantage ratio (selection rate / population rate)
    pub advantage_ratio: f64,
}

impl RoutingAnalysis {
    /// Print a summary of the analysis
    pub fn print_summary(&self) {
        println!("=== Routing Analysis ===");
        println!("High reputation nodes: {} ({:.1}% of population)", 
            self.high_rep_count, self.high_rep_population * 100.0);
        println!("Low reputation nodes: {}", self.low_rep_count);
        println!();
        println!("High reputation selections: {} ({:.1}% of total)", 
            self.high_rep_selections, self.high_rep_rate * 100.0);
        println!("Low reputation selections: {}", self.low_rep_selections);
        println!();
        println!("Advantage ratio: {:.2}x", self.advantage_ratio);
        
        if self.advantage_ratio > 1.0 {
            println!("✓ High reputation nodes are being preferred!");
        } else {
            println!("✗ High reputation nodes are not being preferred");
        }
    }
}
