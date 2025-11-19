use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Material {
    Pla,
    Abs,
    Resin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub file_id: Uuid,
    pub material: Material,
    pub color: String,
    pub layer_height: Option<f64>,     // mm, default 0.2
    pub infill_percentage: Option<i32>, // %, default 20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub id: Uuid,
    pub estimated_cost: f64,
    pub currency: String,
    pub breakdown: CostBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub material_cost: f64,
    pub machine_cost: f64,
    pub labor_cost: f64,
}

/// Calculates the quote based on volume and material.
/// 
/// # Arguments
/// * `volume_cm3` - The volume of the model in cubic centimeters.
/// * `material` - The selected material.
/// 
/// # Returns
/// * `QuoteResponse` containing the calculated cost.
pub fn calculate_quote(volume_cm3: f64, material: &Material) -> QuoteResponse {
    let (density, cost_per_gram) = match material {
        Material::Pla => (1.24, 30.0),
        Material::Abs => (1.04, 40.0),
        Material::Resin => (1.1, 100.0),
    };

    let weight_g = volume_cm3 * density;
    let material_cost = weight_g * cost_per_gram;

    // Simple heuristic: 10 cm3 takes 1 hour
    let print_time_hours = volume_cm3 / 10.0;
    let machine_hourly_rate = 2000.0;
    let machine_cost = print_time_hours * machine_hourly_rate;

    let labor_cost = 0.0; // Not specified in formula but good to have field

    let base_cost = material_cost + machine_cost + labor_cost;
    let markup = 1.5;
    let total_cost = base_cost * markup;

    QuoteResponse {
        id: Uuid::new_v4(),
        estimated_cost: (total_cost * 100.0).round() / 100.0, // Round to 2 decimal places
        currency: "KRW".to_string(),
        breakdown: CostBreakdown {
            material_cost: (material_cost * 100.0).round() / 100.0,
            machine_cost: (machine_cost * 100.0).round() / 100.0,
            labor_cost,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing_logic_pla() {
        let volume = 100.0; // 100 cm3
        let material = Material::Pla;
        
        // Expected calculation:
        // Density PLA = 1.24 g/cm3
        // Weight = 100 * 1.24 = 124 g
        // Material Cost = 124 * 30 KRW/g = 3720 KRW
        // Time = 100 / 10 = 10 hours
        // Machine Cost = 10 * 2000 = 20000 KRW
        // Total Base = 23720
        // Markup 1.5 = 35580
        
        let quote = calculate_quote(volume, &material);
        
        assert_eq!(quote.currency, "KRW");
        assert!((quote.estimated_cost - 35580.0).abs() < 1.0, "Cost should be around 35580, got {}", quote.estimated_cost);
    }

    #[test]
    fn test_pricing_logic_resin() {
        let volume = 50.0; // 50 cm3
        let material = Material::Resin;
        
        // Expected calculation:
        // Density Resin = 1.1 g/cm3
        // Weight = 50 * 1.1 = 55 g
        // Material Cost = 55 * 100 KRW/g = 5500 KRW
        // Time = 50 / 10 = 5 hours
        // Machine Cost = 5 * 2000 = 10000 KRW
        // Total Base = 15500
        // Markup 1.5 = 23250
        
        let quote = calculate_quote(volume, &material);
        
        assert_eq!(quote.currency, "KRW");
        assert!((quote.estimated_cost - 23250.0).abs() < 1.0, "Cost should be around 23250, got {}", quote.estimated_cost);
    }
}
