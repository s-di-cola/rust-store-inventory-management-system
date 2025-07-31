use std::fs;
use serde_json;
use serde_json::from_str;
use serde::{Serialize, Deserialize};
use crate::inventory::Product;
use crate::sales::Sale;
use crate::purchase::Purchase;

const INVENTORY_FILE: &str = "inventory.json";
const SALES_FILE: &str = "sales.json";
const PURCHASES_FILE: &str = "purchases.json";

// Generic load function
fn load_data<T>(filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    if !std::path::Path::new(filename).exists() {
        return Ok(Vec::new());
    }
    let json = fs::read_to_string(filename)?;
    let data: Vec<T> = from_str(&json)?;
    Ok(data)
}

// Generic save function
fn save_data<T>(data: &Vec<T>, filename: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let json = serde_json::to_string_pretty(data)?;
    fs::write(filename, json)?;
    Ok(())
}

// Public API functions using the generic implementations
pub fn load_inventory() -> Result<Vec<Product>, Box<dyn std::error::Error>> {
    load_data(INVENTORY_FILE)
}

pub fn load_sales() -> Result<Vec<Sale>, Box<dyn std::error::Error>> {
    load_data(SALES_FILE)
}

pub fn load_purchases() -> Result<Vec<Purchase>, Box<dyn std::error::Error>> {
    load_data(PURCHASES_FILE)
}

pub fn save_inventory(inventory: &Vec<Product>) -> Result<(), Box<dyn std::error::Error>> {
    save_data(inventory, INVENTORY_FILE)
}

pub fn save_sales(sales: &Vec<Sale>) -> Result<(), Box<dyn std::error::Error>> {
    save_data(sales, SALES_FILE)
}

pub fn save_purchases(purchases: &Vec<Purchase>) -> Result<(), Box<dyn std::error::Error>> {
    save_data(purchases, PURCHASES_FILE)
}
