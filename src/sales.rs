use crate::inventory::Product;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Clone, Debug, Serialize, Deserialize)]
pub struct Sale {
    #[validate(length(min = 1, message = "Product name must be at least 1 character"))]
    pub product_name: String,
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: u32,
    #[validate(range(min = 0.01, message = "Sale price must be greater than 0"))]
    pub sale_price: f64,
    pub profit: f64,
    pub total: f64,
    pub timestamp: DateTime<Utc>,
}

pub trait Sales {
    fn record_sale(
        &mut self,
        product_name: &str,
        quantity: u32,
        sale_price: f64,
        inventory: &mut Vec<Product>,
    ) -> Result<Sale, String>;
}

impl Sales for Vec<Sale> {
    fn record_sale(
        &mut self,
        product_name: &str,
        quantity: u32,
        sale_price: f64,
        inventory: &mut Vec<Product>,
    ) -> Result<Sale, String> {
        let inventory_product = inventory
            .iter_mut()
            .find(|p| p.name == product_name)
            .ok_or_else(|| format!("Product {product_name} not found"))?;

        if inventory_product.quantity < quantity {
            return Err(format!(
                "Insufficient stock for '{}'. Available: {}, Requested: {}",
                inventory_product.name, inventory_product.quantity, quantity
            ));
        }

        inventory_product.quantity -= quantity;

        let sale = Sale {
            product_name: inventory_product.name.clone(),
            quantity,
            sale_price,
            profit: {
                let cost = inventory_product.price * quantity as f64;
                let sale_total = sale_price * quantity as f64;
                sale_total - cost
            },
            total: sale_price * quantity as f64,
            timestamp: Utc::now(),
        };

        sale.validate()
            .map_err(|errors| format!("Validation errors: {errors:#?}"))?;
        self.push(sale.clone());
        Ok(sale)
    }
}
