mod erreurs;
mod action;

use std::env;
use erreurs::Erreur;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        erreurs::handle(Erreur::MauvaiseAction);
    }

    let action = args[0].clone();
    args.remove(0);
    let action = action.as_str();
    match action {
        "create" => action::create::handle(args),
        "delete" => action::delete::handle(args),
        "list" => action::list::handle(args),
        "update" => action::update::handle(args),
        _ => {
            erreurs::handle(Erreur::MauvaiseAction);
            println!("arguments trouvés: {}", args.join(" "));
        }
    }
}