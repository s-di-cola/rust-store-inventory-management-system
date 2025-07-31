use clap::{Parser, Subcommand, ValueEnum};

mod auth;
mod inventory;
mod purchase;
mod report;
mod sales;
mod persistence;

use crate::auth::Auth;
use crate::inventory::{Inventory, Product};
use crate::purchase::{Purchase, Purchases};
use crate::report::Reporter;
use crate::sales::{Sale, Sales};

#[derive(Parser)]
#[command(name = "store management system")]
#[command(about = "An inventory management system for a small retail store")]
#[command(version = "1.0.0")]
#[command(author = "Simone Di Cola")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddProduct {
        name: String,
        #[arg(short, long)]
        price: f64,
        #[arg(short, long)]
        quantity: u32,
        #[arg(short, long)]
        description: String,
    },
    RemoveProduct {
        name: String,
    },
    UpdateProduct {
        name: String,
        #[arg(short, long)]
        price: f64,
        #[arg(short, long)]
        quantity: u32,
        #[arg(short, long)]
        description: String,
    },
    ShowProduct {
        name: String,
    },
    RecordPurchase {
        #[arg(short, long)]
        product_name: String,
        #[arg(short, long)]
        quantity: u32,
        #[arg(short, long)]
        purchase_price: f64,
        #[arg(short, long)]
        description: String,
    },
    RecordSale {
        #[arg(short, long)]
        product_name: String,
        #[arg(short, long)]
        quantity: u32,
        #[arg(short, long)]
        sale_price: f64,
    },
    Report {
        report_type: ReportType,
    },
}

#[derive(ValueEnum, Clone)]
enum ReportType {
    Inventory,
    Sales,
    Purchase,
}

fn main() {
    if let Err(err) = Auth::authenticate() {
        println!("{}", err);
        return;
    };

    println!("=== Store Management System ===");

    let mut inventory: Vec<Product> = Vec::new();
    let mut sales: Vec<Sale> = Vec::new();
    let mut purchases: Vec<Purchase> = Vec::new();

    match Cli::parse().command {
        Commands::AddProduct {
            name,
            price,
            quantity,
            description,
        } => match inventory.add_item(&name, price, quantity, &description) {
            Ok(()) => println!("✓ Product {} successfully added", name),
            Err(err) => println!("⛌ {}", err),
        },
        Commands::RemoveProduct { name } => match inventory.remove_item(&name) {
            Ok(()) => println!("✓ Product {} successfully removed", name),
            Err(err) => println!("⛌ {}", err),
        },
        Commands::UpdateProduct {
            name,
            price,
            quantity,
            description,
        } => match inventory.update_item(&name, price, quantity, &description) {
            Ok(()) => println!("✓ Product {} successfully updated", name),
            Err(err) => println!("⛌ {}", err),
        },
        Commands::ShowProduct { name } => match inventory.get_item(&name) {
            Some(product) => println!("{:?}", product),
            None => println!("Product {} not found", name),
        },
        Commands::RecordPurchase {
            product_name,
            description,
            quantity,
            purchase_price,
        } => {
            match purchases.record_purchase(
                &product_name,
                quantity,
                purchase_price,
                &description,
                &mut inventory,
            ) {
                Ok(purchase) => {
                    println!("✓ Purchase recorded successfully!");
                    println!("  Product: {}", purchase.product_name);
                    println!("  Quantity: {}", purchase.quantity);
                    println!("  Total: ${:.2}", purchase.total_cost);
                }
                Err(err) => println!("⛌ {}", err),
            }
        }
        Commands::RecordSale {
            product_name,
            quantity,
            sale_price,
        } => match sales.record_sale(&product_name, quantity, sale_price, &mut inventory) {
            Ok(sale) => {
                println!("✓ Sale recorded successfully!");
                println!("  Product: {}", sale.product_name);
                println!("  Quantity: {}", sale.quantity);
                println!("  Sale Price: ${:.2}", sale.sale_price);
                println!("  Profit: ${:.2}", sale.profit);
            }
            Err(err) => println!("⛌ {}", err),
        },
        Commands::Report { report_type } => match report_type {
            ReportType::Inventory => {
                let report = Reporter::generate_inventory_report(&inventory);
                println!("{}", report);
            }
            ReportType::Sales => {
                let report = Reporter::generate_sales_report(&sales);
                println!("{}", report);
            }
            ReportType::Purchase => {
                let report = Reporter::generate_purchase_report(&purchases);
                println!("{}", report);
            }
        },
    }
}
