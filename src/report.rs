use crate::inventory::Product;
use crate::purchase::Purchase;
use crate::sales::Sale;

pub struct Reporter;

impl Reporter {
    pub fn generate_inventory_report(inventory: &Vec<Product>) -> String {
        let mut report = String::from("INVENTORY REPORT\n");
        report.push_str("================\n");

        for product in inventory {
            report.push_str(&format!(
                "Product: {} | Price: ${:.2} | Qty: {} | Desc: {}\n",
                product.name, product.price, product.quantity, product.description
            ));
        }

        report.push_str(&format!("\nTotal Products: {}\n", inventory.len()));
        report
    }

    pub fn generate_sales_report(sales: &Vec<Sale>) -> String {
        let mut report = String::from("SALES REPORT\n");
        report.push_str("============\n");

        for sale in sales {
            report.push_str(&format!(
                "Product: {} | Qty: {} | Price: ${:.2} | Total: ${:.2} | Profit: ${:.2}\n",
                sale.product_name, sale.quantity, sale.sale_price, sale.total, sale.profit
            ));
        }

        let total_sales: f64 = sales.iter().map(|s| s.total).sum();
        let total_profit: f64 = sales.iter().map(|s| s.profit).sum();
        report.push_str(&format!("\nTotal Sales: ${:.2} | Total Profit: ${:.2}\n", total_sales, total_profit));
        report
    }

    pub fn generate_purchase_report(purchases: &Vec<Purchase>) -> String {
        let mut report = String::from("PURCHASE REPORT\n");
        report.push_str("===============\n");

        for purchase in purchases {
            report.push_str(&format!(
                "Product: {} | Qty: {} | Unit Price: ${:.2} | Total: ${:.2}\n",
                purchase.product_name, purchase.quantity, purchase.purchase_price, purchase.total_cost
            ));
        }

        let total_cost: f64 = purchases.iter().map(|p| p.total_cost).sum();
        report.push_str(&format!("\nTotal Purchase Cost: ${:.2}\n", total_cost));
        report
    }
}