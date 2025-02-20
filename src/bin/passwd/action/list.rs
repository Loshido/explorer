use explorer::auth::fs::passwd;

pub fn handle(_args: Vec<String>) {
    let users = passwd();

    println!("utilisateurs: ");
    for user in users.0.iter() {
        println!(" - {}", user.0.0);
    }
}