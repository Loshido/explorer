use explorer::auth::fs::passwd;
use crate::erreurs::{self, Erreur};

pub fn handle(args: Vec<String>) {
    if args.len() != 2 {
        let erreur = Erreur::MauvaisArguments(
            format!("create <utilisateur> <mots de passe>")
        );
        erreurs::handle(erreur);
        return;
    }

    let username = args[0].as_str().trim();
    let password = args[1].as_str().trim();
    let mut users = passwd();

    let success = users.create(username.to_string(), password.to_string());
    match success {
        true => {
            println!("L'utilisateur {} a été créé avec succès", username)
        },
        false => {
            println!("L'utilisateur {} n'a pas été créé", username)
        }
    }
}