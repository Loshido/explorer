use explorer::auth::fs::passwd;
use crate::erreurs::{self, Erreur};

pub fn handle(args: Vec<String>) {
    if args.len() != 2 {
        let erreur = Erreur::MauvaisArguments(
            format!("create <utilisateur> <utilisateur | mdp> <valeur>")
        );
        erreurs::handle(erreur);
        return;
    }

    let user = args[0].as_str().trim();
    let entry = args[1].as_str().trim();
    let value = args[2].as_str().trim();
    let mut users = passwd();

    let user = users.0.iter_mut()
        .find(|v| v.0.0 == user.to_string());

    if user.is_some() {
        let user = user.unwrap();
        match entry {
            "utilisateur" => {
                user.0.0 = value.to_string();
                users.save();
                println!("Le nom de {} a été modifié avec succès.", args[0].clone());
            },
            "mdp" => {
                user.0.1 = value.to_string();
                users.save();
                println!("Le mots de passe de {} a été modifié avec succès.", args[0].clone());
            },
            _ => {
                let erreur = Erreur::MauvaisArguments(
                    format!("create <utilisateur> <utilisateur | mdp> <valeur>")
                );
                erreurs::handle(erreur);
                return;
            }
        }
    } else {
        erreurs::handle(Erreur::Introuvable(args[0].clone()));
        return;
    }
}