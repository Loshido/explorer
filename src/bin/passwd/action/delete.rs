use explorer::auth::fs::passwd;
use crate::erreurs::{self, Erreur};

pub fn handle(args: Vec<String>) {
    if args.len() != 1 {
        let erreur = Erreur::MauvaisArguments(
            format!("delete <utilisateur>")
        );
        erreurs::handle(erreur);
        return;
    }

    let username = args[0].as_str();
    let mut users = passwd();

    let success = users.remove(username.to_string());
    match success {
        true => {
            println!("L'utilisateur {} a été supprimé avec succès", username)
        },
        false => {
            println!("L'utilisateur {} n'a pas été supprimé", username)
        }
    }
}