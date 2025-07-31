# Rust Store Inventory Management System

A comprehensive command-line inventory management system for small retail stores, built with Rust. This system provides secure authentication, product management, sales tracking, purchase recording, detailed reporting capabilities, and persistent data storage.

## Features

- **Authentication**: Secure login system for store managers
- **Product Management**: Add, update, remove, and view products with validation
- **Inventory Tracking**: Real-time inventory quantity management
- **Sales Recording**: Track sales with automatic profit calculations
- **Purchase Management**: Record purchases and automatically update inventory
- **Reporting**: Generate detailed reports with human-readable timestamps
- **Data Persistence**: JSON-based storage in organized `data/` directory
- **Data Validation**: Input validation using the `validator` crate
- **CLI Interface**: User-friendly command-line interface with short and long flags

## Installation

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust-store-inventory-management-system
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run -- --help
   ```

## Usage

### Authentication
The system requires authentication before accessing any functionality:
- **Username**: `store_manager`
- **Password**: `pass1234`

### Commands

#### Add Product
Add a new product to the inventory:
```bash
cargo run -- add-product --name "Product Name" --price 19.99 --quantity 100 --description "Product description"
# Or using short flags:
cargo run -- add-product -n "Product Name" -p 19.99 -q 100 -d "Product description"
```

#### Update Product
Update an existing product's details:
```bash
cargo run -- update-product --name "Product Name" --price 24.99 --quantity 150 --description "Updated description"
# Or using short flags:
cargo run -- update-product -n "Product Name" -p 24.99 -q 150 -d "Updated description"
```

#### Remove Product
Remove a product from inventory:
```bash
cargo run -- remove-product "Product Name"
```

#### View Product
View details of a specific product:
```bash
cargo run -- show-product --name "Product Name"
# Or using short flags:
cargo run -- show-product -n "Product Name"
```

#### Record Purchase
Record a purchase and automatically update inventory:
```bash
cargo run -- record-purchase --product-name "Product Name" --quantity 50 --purchase-price 15.00 --description "Supplier ABC"
# Or using short flags:
cargo run -- record-purchase -n "Product Name" -q 50 -p 15.00 -d "Supplier ABC"
```

#### Record Sale
Record a sale and update inventory:
```bash
cargo run -- record-sale --product-name "Product Name" --quantity 5 --sale-price 25.00
# Or using short flags:
cargo run -- record-sale -n "Product Name" -q 5 -s 25.00
```

#### Generate Reports
Generate various types of reports with human-readable timestamps:

**Inventory Report:**
```bash
cargo run -- report --report-type inventory
# Or using short flags:
cargo run -- report -r inventory
```

**Sales Report:**
```bash
cargo run -- report --report-type sales
# Or using short flags:
cargo run -- report -r sales
```

**Purchase Report:**
```bash
cargo run -- report --report-type purchase
# Or using short flags:
cargo run -- report -r purchase
```

## Project Structure

```
src/
├── main.rs          # Main application entry point and CLI handling
├── auth.rs          # Authentication module
├── inventory.rs     # Product and inventory management
├── sales.rs         # Sales recording and tracking
├── purchase.rs      # Purchase recording and inventory updates
├── report.rs        # Report generation with formatted timestamps
└── persistence.rs   # Generic data persistence with JSON storage

data/                # Data storage directory (auto-created)
├── inventory.json   # Product inventory data
├── sales.json       # Sales transaction history
└── purchases.json   # Purchase transaction history
```

## Data Models

### Product
- `name`: String (1-50 characters)
- `price`: f64 (positive value)
- `quantity`: u32
- `description`: String

### Sale
- `product_name`: String
- `quantity`: u32
- `sale_price`: f64
- `profit`: f64 (calculated automatically)
- `total`: f64 (calculated automatically)
- `timestamp`: DateTime<Utc> (formatted as "YYYY-MM-DD HH:MM:SS UTC")

### Purchase
- `product_name`: String
- `quantity`: u32
- `purchase_price`: f64
- `total_cost`: f64 (calculated automatically)
- `timestamp`: DateTime<Utc> (formatted as "YYYY-MM-DD HH:MM:SS UTC")

## Dependencies

- `clap`: Command-line argument parsing with short and long flags
- `validator`: Data validation with custom error messages
- `serde` & `serde_json`: JSON serialization and deserialization
- `chrono`: Date and time handling with human-readable formatting

## Data Storage

- **Location**: All data files are stored in the `data/` directory
- **Format**: JSON files for easy inspection and portability
- **Auto-creation**: The `data/` directory is created automatically if it doesn't exist
- **Persistence**: Data is automatically saved after each operation

## Error Handling

The system includes comprehensive error handling for:
- Invalid authentication credentials
- Product not found scenarios
- Insufficient inventory for sales
- Data validation failures
- File I/O operations
- JSON parsing errors

## Security Features

- Password-protected access with authentication requirement
- Input validation and sanitization for all user inputs
- Safe memory management with Rust's ownership system
- Proper error handling without exposing sensitive information

## CLI Short Flags

For faster usage, the following short flags are available:
- `-n, --name` or `--product-name`: Product name
- `-p, --price` or `--purchase-price`: Price/purchase price
- `-q, --quantity`: Quantity
- `-d, --description`: Description
- `-s, --sale-price`: Sale price
- `-r, --report-type`: Report type (inventory, sales, purchase)

## Example Output

### Sales Report
```
SALES REPORT
============
Product: Apple | Qty: 5 | Price: $1.50 | Total: $7.50 | Profit: $2.50 | Date: 2025-07-31 14:39:43 UTC

Total Sales: $7.50 | Total Profit: $2.50
