use validator::Validate;

#[derive(Validate)]
pub struct Product {
    #[validate(length(min = 1, max = 50, message = "Product name must be 1-50 characters"))]
    pub name: String,
    #[validate(range(min = 0.0, message = "Price must be greater than 0"))]
    pub price: f64,
    #[validate(range(min = 0, message = "Quantity must be greater than 0"))]
    pub quantity: u32,
    #[validate(length(min = 1, max = 255, message = "Description must be 1-255 characters"))]
    pub description: String,
}

trait Inventory {
    fn add_item(
        &mut self,
        name: &str,
        price: f64,
        quantity: u32,
        description: &str,
    ) -> Result<(), String>;
    fn remove_item(&mut self, name: &str) -> Result<(), String>;
    fn update_item(
        &mut self,
        name: &str,
        price: f64,
        quantity: u32,
        description: &str,
    ) -> Result<(), String>;
    fn get_item(&self, name: &str) -> Option<&Product>;
}

impl Inventory for Vec<Product> {
    fn add_item(
        &mut self,
        name: &str,
        price: f64,
        quantity: u32,
        description: &str,
    ) -> Result<(), String> {
        let product = Product {
            name: name.to_string(),
            price,
            quantity,
            description: description.to_string(),
        };

        if self.iter().any(|p| p.name == name) {
            return Err(format!("Product {} already exists", name));
        };

        product
            .validate()
            .map_err(|errors| format!("Validation errors: {:#?}", errors))?;

        self.push(product);
        Ok(())
    }

    fn remove_item(&mut self, name: &str) -> Result<(), String> {
        match self.iter().position(|p| p.name == name) {
            Some(index) => {
                self.remove(index);
                Ok(())
            }
            None => Err(format!("Product {} not found", name)),
        }
    }

    fn update_item(
        &mut self,
        name: &str,
        price: f64,
        quantity: u32,
        description: &str,
    ) -> Result<(), String> {
        let temp_product = Product {
            name: name.to_string(),
            price,
            quantity,
            description: description.to_string(),
        };

        temp_product
            .validate()
            .map_err(|errors| format!("Validation errors: {:#?}", errors))?;

        match self.iter_mut().find(|p| p.name == name) {
            Some(product) => {
                product.price = price;
                product.quantity = quantity;
                product.description = description.to_string();
                Ok(())
            }
            None => Err(format!("Product {} not found", name)),
        }
    }

    fn get_item(&self, name: &str) -> Option<&Product> {
        self.iter().find(|p| p.name == name)
    }
}
