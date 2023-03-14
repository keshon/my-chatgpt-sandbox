use chrono::Utc;
use rand::Rng;
use uuid::Uuid;
use std::io::{self, Write};

// Define the inventory struct
#[derive(Debug)]
struct Inventory {
    items: Vec<Item>,
}

// Define the item struct
#[derive(Debug)]
struct Item {
    id: u128,
    name: String,
    price: f64,
}

impl Inventory {
    // Create a new item and add it to the inventory
    fn create(&mut self, name: String, price: f64) -> u128 {
        // let id = generate_unique_id(); // Generate a unique ID for the new item
        let id = generate_uuid(); // Generate a unique ID for the new item
        let item = Item {
            id,
            name,
            price,
        };
        self.items.push(item); // Add the new item to the inventory
        id // Return the unique ID for the new item
    }

    // Find an item in the inventory by its ID and return a reference to it
    fn read(&self, id: u128) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }

    // Update the price of an item in the inventory by its ID
    fn update(&mut self, id: u128, price: f64) -> bool {
        match self.items.iter_mut().find(|item| item.id == id) {
            Some(item) => {
                item.price = price;
                true
            }
            None => false,
        }
    }

    // Remove an item from the inventory by its ID
    fn delete(&mut self, id: u128) -> bool {
        match self.items.iter().position(|item| item.id == id) {
            Some(index) => {
                self.items.remove(index);
                true
            }
            None => false,
        }
    }

    // Print out all items in the inventory
    fn show_all(&self) {
        for item in &self.items {
            println!("{:?}", item);
        }
    }
}

// Generate a unique ID for a new item
fn generate_unique_id() -> u128 {
    let now = Utc::now().timestamp_nanos(); // Get the current time in nanoseconds
    let rand_num: u128 = rand::thread_rng().gen(); // Generate a random number
    now as u128 ^ rand_num // Combine the time and random number to create a unique ID
}
// Generates a version 4 UUID, which is a randomly generated UUID.
fn generate_uuid() -> u128 {
    Uuid::new_v4().as_u128()
}

fn main() {
    // Create a new inventory
    let mut inventory = Inventory { items: vec![] };

    // Loop through the program until the user chooses to exit
    loop {
        // Print out the main menu
        println!("Welcome to the inventory management system!");
        println!("Please select an option:");
        println!("1. Create new item");
        println!("2. Read item");
        println!("3. Update item");
        println!("4. Delete item");
        println!("5. Show all items");
        println!("6. Exit");

        // Get the user's choice
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Perform the appropriate action based on the user's choice
        match choice {
            1 => {
                // Create new item
                print!("Enter new item name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&            mut name).unwrap();
                name = name.trim().to_string();
                print!("Enter new item price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid price");
                        continue;
                    }
                };
                let id = inventory.create(name, price);
                println!("Created new item with ID: {}", id);
            }
            2 => {
                // Read item
                print!("Enter item ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u128 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };
                match inventory.read(id) {
                    Some(item) => println!("{:?}", item),
                    None => println!("Item not found"),
                }
            }
            3 => {
                // Update item
                print!("Enter item ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u128 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };
                print!("Enter new price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid price");
                        continue;
                    }
                };
                if inventory.update(id, price) {
                    println!("Item updated");
                } else {
                    println!("Item not found");
                }
            }
            4 => {
                // Delete item
                print!("Enter item ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u128 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };
                if inventory.delete(id) {
                    println!("Item deleted");
                } else {
                    println!("Item not found");
                }
            }
            5 => {
                // Show all items
                inventory.show_all();
            }
            6 => {
                // Exit the program
                println!("Goodbye!");
                break;
            }
            _ => {
                // Invalid choice
                println!("Invalid choice");
            }
        }
    }
}    
