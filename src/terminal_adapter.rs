use std::io;

pub fn lire_ligne_simple() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn lire_texte_arabe() -> String {
    let ligne = lire_ligne_simple();

    ligne.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn lire_racine_terminal() -> Option<[char; 3]> {
    println!("Entrez la racine (3 lettres séparées par des espaces, ex: ك ت ب) :");
    let ligne = lire_ligne_simple();
    let chars: Vec<char> = ligne.chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() == 3 {
        Some([chars[0], chars[1], chars[2]])
    } else {
        println!("Erreur : vous devez entrer exactement 3 lettres.");
        None
    }
}

pub fn afficher_arabe(texte: &str) -> String {
    texte.chars().rev().collect()
}
