use crate::inventory::Product;
use std::time::SystemTime;
use validator::Validate;

#[derive(Validate, Clone)]
pub struct Purchase {
    #[validate(length(min = 1, message = "Product name cannot be empty"))]
    pub product_name: String,
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: u32,
    #[validate(range(min = 0.01, message = "Purchase price must be at least 0.01"))]
    pub purchase_price: f64,
    pub(crate) timestamp: SystemTime,
    pub(crate) total_cost: f64,
}

pub trait Purchases {
    fn record_purchase(
        &mut self,
        product_name: &str,
        quantity: u32,
        purchase_price: f64,
        description: &str,
        inventory: &mut Vec<Product>,
    ) -> Result<Purchase, String>;
    fn get_total_purchases(&self) -> f64;
    fn get_purchases(&self, product_name: &str) -> Option<Vec<Purchase>>;
}

impl Purchases for Vec<Purchase> {
    fn record_purchase(
        &mut self,
        product_name: &str,
        quantity: u32,
        purchase_price: f64,
        description: &str,
        inventory: &mut Vec<Product>,
    ) -> Result<Purchase, String> {
        let purchase = Purchase {
            product_name: product_name.to_string(),
            quantity,
            purchase_price,
            timestamp: SystemTime::now(),
            total_cost: purchase_price * quantity as f64,
        };
        purchase
            .validate()
            .map_err(|errors| format!("Validation errors: {:#?}", errors))?;

        match inventory.iter_mut().find(|p| p.name == product_name) {
            Some(product) => {
                product.quantity += quantity;
            },
            None{} => {
                let new_product = Product {
                    name: product_name.to_string(),
                    price: purchase_price,
                    quantity,
                    description: description.to_string(),
                };

                new_product
                    .validate()
                    .map_err(|errors| format!("Validation errors: {:#?}", errors))?;

                inventory.push(new_product);
            }
        }
        self.push(purchase.clone());
        Ok(purchase)
    }

    fn get_total_purchases(&self) -> f64 {
        self.iter()
            .fold(0.0, |acc, purchase| acc + purchase.total_cost)
    }

    fn get_purchases(&self, product_name: &str) -> Option<Vec<Purchase>> {
        let purchases: Vec<Purchase> = self
            .iter()
            .filter(|p| p.product_name == product_name)
            .cloned()
            .collect();

        match purchases.is_empty() {
            true => None,
            false => Some(purchases),
        }
    }
}
