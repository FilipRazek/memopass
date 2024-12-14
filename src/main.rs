use std::io;

fn main() {
    // Enter password - DONE
    // Decompose password based on association files. use a greedy algorithm, exact case matching
    // Print out the result
    // ---- MVP ---
    // Define a format for association files
    // Create a few examples
    // Add tests and behavior
    // Add cli option to supply files
    // Add password generator (cli option)
    // Add option to score password (https://crates.io/crates/passwords)
    // ---- DOD ----
    print_banner();

    let password = read_password();
}

fn print_banner() {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    println!("Memopass v{} - Memorize your passwords", VERSION.unwrap_or(".<unknown>"));
    println!();
}

fn read_password() -> String {
    println!("Input your password:");
    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password");

    password.trim().to_string()
}