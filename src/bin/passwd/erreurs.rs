pub enum Erreur {
    MauvaiseAction,
    MauvaisArguments(String),
    Introuvable(String)
}

pub fn handle(error: Erreur) {
    match error {
        Erreur::MauvaiseAction => {
            println!("Mauvaise utilisation du programme!");
            println!("passwd <create | list | delete | update> ...");
        },
        Erreur::MauvaisArguments(arguments) => {
            println!("Mauvais arguments");
            println!("passwd {}", arguments);
        },
        Erreur::Introuvable(utilisateur) => {
            println!("Utilisateur {} introuvable!", utilisateur)
        }
    }
}