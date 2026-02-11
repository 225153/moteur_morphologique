// ============================================================================
// MODULE TERMINAL ADAPTER
// ============================================================================
// Ce module contient TOUTES les fonctions spécifiques au terminal Windows
// qui gère mal le RTL arabe.
//
// À SUPPRIMER lors de la migration vers web/mobile !
// ============================================================================

use std::io;

// Lire une ligne de texte depuis le terminal
pub fn lire_ligne_simple() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Lire du texte arabe depuis le terminal (sans inversion)
// Le terminal Windows affiche en LTR mais on stocke en RTL
pub fn lire_texte_arabe() -> String {
    let ligne = lire_ligne_simple();
    // Enlever les espaces seulement
    ligne.chars().filter(|c| !c.is_whitespace()).collect()
}

// Lire une racine de 3 caractères arabes depuis le terminal
pub fn lire_racine_terminal() -> Option<[char; 3]> {
    println!("Entrez la racine (3 lettres séparées par des espaces, ex: ك ت ب) :");
    let ligne = lire_ligne_simple();
    let chars: Vec<char> = ligne.chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() == 3 {
        // Ne PAS inverser - stocker tel quel en RTL
        Some([chars[0], chars[1], chars[2]])
    } else {
        println!("Erreur : vous devez entrer exactement 3 lettres.");
        None
    }
}

// Afficher du texte arabe correctement dans le terminal Windows
// Le terminal affiche en LTR, donc on inverse pour simuler RTL
pub fn afficher_arabe(texte: &str) -> String {
    texte.chars().rev().collect()
}
