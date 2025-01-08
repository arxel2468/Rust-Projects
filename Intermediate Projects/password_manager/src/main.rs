use bcrypt::{hash, verify, DEFAULT_COST};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)] // Add Clone trait
struct Credential {
    service: String,
    username: String,
    password_hash: String,
}

impl Credential {
    fn new(service: &str, username: &str, password: &str) -> Self {
        let password_hash = hash(password, DEFAULT_COST).unwrap(); // Hash the password with bcrypt
        Credential {
            service: service.to_string(),
            username: username.to_string(),
            password_hash,
        }
    }

    fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap() // Verify password with the hash
    }
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Welcome to your password manager!");
        println!("1. Add Credential");
        println!("2. Verify Credential");
        println!("3. Exit");

        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => add_credential(),
            "2" => verify_credential(),
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_credential() {
    let mut service = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter the service name (e.g., Gmail): ");
    io::stdin().read_line(&mut service).unwrap();
    println!("Enter your username: ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Enter your password: ");
    io::stdin().read_line(&mut password).unwrap();

    let credential = Credential::new(&service.trim(), &username.trim(), &password.trim());
    save_credential(&credential);
    println!("Credential added successfully!");
}

fn verify_credential() {
    let mut service = String::new();
    let mut password = String::new();

    println!("Enter the service name to verify: ");
    io::stdin().read_line(&mut service).unwrap();
    println!("Enter your password: ");
    io::stdin().read_line(&mut password).unwrap();

    let credentials = load_credentials();
    let mut found = false;

    for cred in credentials {
        if cred.service == service.trim() {
            found = true;
            if cred.verify_password(&password.trim()) {
                println!("Password verified for service {}", service.trim());
            } else {
                println!("Incorrect password for {}", service.trim());
            }
        }
    }

    if !found {
        println!("No credentials found for service {}", service.trim());
    }
}

fn save_credential(credential: &Credential) {
    let mut credentials = load_credentials();
    credentials.push(credential.clone());

    let file = File::create("credentials.json").unwrap();
    serde_json::to_writer(file, &credentials).unwrap();
}

fn load_credentials() -> Vec<Credential> {
    let mut file = match File::open("credentials.json") {
        Ok(f) => f,
        Err(_) => return Vec::new(), // Return empty vector if file doesn't exist
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()) // Return empty vector if parsing fails
}
