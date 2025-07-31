use crate::inventory::Product;
use std::time::SystemTime;
use validator::Validate;

#[derive(Validate, Clone)]
pub struct Sale {
    #[validate(length(min = 1, message = "Product name must be at least 1 character"))]
    product_name: String,
    #[validate(range(min = 0.0, message = "Sale price must be greater than 0"))]
    sale_price: f64,
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    quantity: u32,
    profit: f64,
    total: f64,
    timestamp: SystemTime,
}

trait Sales {
    fn record_sale(
        &mut self,
        product: &Product,
        quantity: u32,
        sale_price: f64,
    ) -> Result<Sale, String>;

    fn get_total_sales(&self) -> f64;
    fn get_total_profit(&self) -> f64;
}

impl Sales for Vec<Sale> {
    fn record_sale(
        &mut self,
        product: &Product,
        quantity: u32,
        sale_price: f64,
    ) -> Result<Sale, String> {
        let sale = Sale {
            product_name: product.name.clone(),
            quantity,
            sale_price,
            profit: {
                let cost = product.price * quantity as f64;
                let sale_total = sale_price * quantity as f64;
                sale_total - cost
            },
            total: sale_price * quantity as f64,
            timestamp: SystemTime::now(),
        };

        sale.validate()
            .map_err(|errors| format!("Validation errors: {:#?}", errors))?;
        self.push(sale.clone());
        Ok(sale)
    }

    fn get_total_sales(&self) -> f64 {
        self.iter().fold(0.0, |acc, sale| acc + sale.total)
    }

    fn get_total_profit(&self) -> f64 {
        self.iter().fold(0.0, |acc, sale| acc + sale.profit)
    }
}
