use crate::inventory::Product;
use crate::purchase::Purchase;
use crate::sales::Sale;

pub struct Reporter;

impl Reporter {
    pub fn generate_inventory_report(inventory: &[Product]) -> String {
        let mut report = String::from("INVENTORY REPORT\n================\n");

        if inventory.is_empty() {
            report.push_str("No products in inventory.\n");
            return report;
        }

        for product in inventory {
            report.push_str(&format!(
                "Product: {} | Price: ${:.2} | Qty: {} | Description: {}\n",
                product.name, product.price, product.quantity, product.description
            ));
        }

        let total_value: f64 = inventory.iter().map(|p| p.price * p.quantity as f64).sum();
        let total_items: u32 = inventory.iter().map(|p| p.quantity).sum();
        report.push_str(&format!(
            "\nTotal Items: {total_items} | Total Value: ${total_value:.2}\n"
        ));
        report
    }

    pub fn generate_sales_report(sales: &[Sale]) -> String {
        let mut report = String::from("SALES REPORT\n============\n");

        if sales.is_empty() {
            report.push_str("No sales recorded.\n");
            return report;
        }

        for sale in sales {
            report.push_str(&format!(
                "Product: {} | Qty: {} | Price: ${:.2} | Total: ${:.2} | Profit: ${:.2} | Date: {}\n",
                sale.product_name,
                sale.quantity,
                sale.sale_price,
                sale.total,
                sale.profit,
                sale.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }

        let total_sales: f64 = sales.iter().map(|s| s.total).sum();
        let total_profit: f64 = sales.iter().map(|s| s.profit).sum();
        report.push_str(&format!(
            "\nTotal Sales: ${total_sales:.2} | Total Profit: ${total_profit:.2}\n"
        ));
        report
    }

    pub fn generate_purchase_report(purchases: &[Purchase]) -> String {
        let mut report = String::from("PURCHASE REPORT\n===============\n");

        if purchases.is_empty() {
            report.push_str("No purchases recorded.\n");
            return report;
        }

        for purchase in purchases {
            report.push_str(&format!(
                "Product: {} | Qty: {} | Unit Price: ${:.2} | Total: ${:.2} | Date: {}\n",
                purchase.product_name,
                purchase.quantity,
                purchase.purchase_price,
                purchase.total_cost,
                purchase.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }

        let total_cost: f64 = purchases.iter().map(|p| p.total_cost).sum();
        report.push_str(&format!("\nTotal Purchase Cost: ${total_cost:.2}\n"));
        report
    }
}
