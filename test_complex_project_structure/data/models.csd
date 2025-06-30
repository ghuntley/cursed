// Data models and structures
export User, Product, Order, ValidationError

// User model with methods
struct User {
    name: string,
    age: int,
    email: string,
    is_active: bool
}

impl User {
    func new(name: string, age: int) -> User {
        return User {
            name: name,
            age: age,
            email: name.to_lowercase() + "@example.com",
            is_active: true
        }
    }
    
    func is_adult(&self) -> bool {
        return self.age >= 18
    }
    
    func update_email(&mut self, new_email: string) -> Result<(), ValidationError> {
        if !validate_email(new_email) {
            return Err(ValidationError.new("Invalid email format"))
        }
        self.email = new_email
        return Ok(())
    }
    
    func to_json(&self) -> string {
        return "{\"name\":\"" + self.name + "\",\"age\":" + self.age.to_string() + ",\"email\":\"" + self.email + "\"}"
    }
}

// Product model
struct Product {
    id: int,
    name: string,
    price: float,
    category: string,
    in_stock: int
}

impl Product {
    func new(id: int, name: string, price: float, category: string) -> Product {
        return Product {
            id: id,
            name: name,
            price: price,
            category: category,
            in_stock: 0
        }
    }
    
    func is_available(&self) -> bool {
        return self.in_stock > 0
    }
    
    func update_stock(&mut self, quantity: int) {
        self.in_stock = quantity
    }
}

// Order model with dependency on User and Product
struct Order {
    id: int,
    user: User,
    products: Vec<Product>,
    total: float,
    status: string
}

impl Order {
    func new(id: int, user: User) -> Order {
        return Order {
            id: id,
            user: user,
            products: Vec.new(),
            total: 0.0,
            status: "pending"
        }
    }
    
    func add_product(&mut self, product: Product, quantity: int) -> Result<(), ValidationError> {
        if !product.is_available() {
            return Err(ValidationError.new("Product not available"))
        }
        
        if product.in_stock < quantity {
            return Err(ValidationError.new("Insufficient stock"))
        }
        
        for _ in range(0, quantity) {
            self.products.push(product.clone())
        }
        
        self.calculate_total()
        return Ok(())
    }
    
    func calculate_total(&mut self) {
        self.total = 0.0
        for product in &self.products {
            self.total += product.price
        }
    }
    
    func complete_order(&mut self) -> Result<(), ValidationError> {
        if self.products.is_empty() {
            return Err(ValidationError.new("Cannot complete empty order"))
        }
        
        self.status = "completed"
        return Ok(())
    }
}

// Custom error type
struct ValidationError {
    message: string
}

impl ValidationError {
    func new(message: string) -> ValidationError {
        return ValidationError { message: message }
    }
}

// Helper functions
func validate_email(email: string) -> bool {
    return email.contains("@") && email.contains(".")
}
