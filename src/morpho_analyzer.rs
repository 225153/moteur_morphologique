// Moteur de dérivation morphologique
// Ce module contient le cœur du projet : générer des mots à partir d'une racine et d'un schème

// Fonction principale : générer un mot dérivé
// Elle prend une racine [char; 3] et un nom de schème (String)
// Elle remplace ف par la 1ère lettre, ع par la 2ème, ل par la 3ème
//
// Exemple : racine = ['ك', 'ت', 'ب'], schème = "فاعل"
//   ف → ك, ع → ت, ل → ب  ==>  résultat = "كاتب"

pub fn generer_mot(racine: [char; 3], scheme_nom: &str) -> String {
    let mut resultat = String::new();

    // Parcourir chaque caractère du schème
    for c in scheme_nom.chars() {
        if c == 'ف' {
            // Remplacer ف par la 1ère consonne de la racine
            resultat.push(racine[0]);
        } else if c == 'ع' {
            // Remplacer ع par la 2ème consonne de la racine
            resultat.push(racine[1]);
        } else if c == 'ل' {
            // Remplacer ل par la 3ème consonne de la racine
            resultat.push(racine[2]);
        } else {
            // Garder le caractère tel quel (ا, م, ت, و, etc.)
            resultat.push(c);
        }
    }

    resultat
}

// Générer TOUS les dérivés d'une racine à partir d'une liste de schèmes
// Retourne un vecteur de (nom_du_schème, mot_généré)
pub fn generer_famille(racine: [char; 3], schemes: &[(&str, &str)]) -> Vec<(String, String)> {
    let mut resultats: Vec<(String, String)> = Vec::new();

    // Pour chaque schème, on génère le mot correspondant
    for (nom, description) in schemes {
        let mot = generer_mot(racine, nom);
        resultats.push((nom.to_string(), mot));
    }

    resultats
}

// Afficher toute la famille morphologique d'une racine
pub fn afficher_famille(racine: [char; 3], schemes: &[(&str, &str)]) {
    let r: String = racine.iter().collect();
    println!("=== Famille morphologique de {} ===", r);

    let famille = generer_famille(racine, schemes);

    for (scheme, mot) in &famille {
        println!("  {} → {}", scheme, mot);
    }

    println!("Total: {} dérivés", famille.len());
}

// ========================================================
// VALIDATION MORPHOLOGIQUE
// ========================================================

// Vérifier si un mot appartient à une racine donnée
// Principe : on essaie chaque schème, on génère le mot, et on compare
//
// Exemple : valider("مكتوب", ['ك','ت','ب'], schemes)
//   On essaie فاعل → كاتب ≠ مكتوب
//   On essaie مفعول → مكتوب == مكتوب  ✅ trouvé !
//   Retourne (true, Some("مفعول"))

pub fn valider_mot(
    mot: &str,
    racine: [char; 3],
    schemes: &[(&str, &str)],
) -> (bool, Option<String>) {
    // Parcourir chaque schème
    for (nom_scheme, _description) in schemes {
        // Générer le mot avec ce schème (on réutilise generer_mot)
        let mot_genere = generer_mot(racine, nom_scheme);

        // Comparer le mot généré avec le mot à valider
        if mot_genere == mot {
            // Trouvé ! Le mot correspond à ce schème
            return (true, Some(nom_scheme.to_string()));
        }
    }

    // Aucun schème ne correspond
    (false, None)
}

// Version avec affichage : vérifie et affiche le résultat
pub fn afficher_validation(mot: &str, racine: [char; 3], schemes: &[(&str, &str)]) {
    let r: String = racine.iter().collect();
    let (trouve, scheme) = valider_mot(mot, racine, schemes);

    if trouve {
        println!("✓ OUI : '{}' appartient à la racine '{}'", mot, r);
        println!("  Schème utilisé : {}", scheme.unwrap());
    } else {
        println!("✗ NON : '{}' n'appartient pas à la racine '{}'", mot, r);
    }
}

// ========================================================
// GESTION DES DÉRIVÉS VALIDÉS (lien arbre ↔ dérivation)
// ========================================================

use crate::arbre::Tree;

// Générer tous les dérivés d'une racine ET les stocker dans l'arbre
// Retourne le nombre de dérivés ajoutés
pub fn generer_et_stocker(arbre: &mut Tree, racine: [char; 3], schemes: &[(&str, &str)]) -> u32 {
    let mut compteur: u32 = 0;

    // Pour chaque schème, on génère le mot et on le stocke dans l'arbre
    for (nom_scheme, _description) in schemes {
        let mot = generer_mot(racine, nom_scheme);

        // Stocker dans le nœud de la racine dans l'arbre
        let ok = arbre.ajouter_derive(racine, mot.clone(), nom_scheme.to_string());
        if ok {
            compteur = compteur + 1;
        }
    }

    let r: String = racine.iter().collect();
    println!(
        "{} dérivés générés et stockés pour la racine '{}'",
        compteur, r
    );
    compteur
}

// Valider un mot ET le stocker si valide
// Retourne (trouvé, schème trouvé)
pub fn valider_et_stocker(
    arbre: &mut Tree,
    mot: &str,
    racine: [char; 3],
    schemes: &[(&str, &str)],
) -> (bool, Option<String>) {
    let (trouve, scheme) = valider_mot(mot, racine, schemes);

    if trouve {
        // Le mot est valide → on le stocke dans l'arbre
        let schema = scheme.clone().unwrap();
        arbre.ajouter_derive(racine, mot.to_string(), schema);
    }

    (trouve, scheme)
}

// Afficher les dérivés stockés pour une racine dans l'arbre
pub fn afficher_derives_stockes(arbre: &mut Tree, racine: [char; 3]) {
    let noeud = arbre.chercher_noeud(racine);
    match noeud {
        Some(n) => n.afficher_derives(),
        None => {
            let r: String = racine.iter().collect();
            println!("Racine '{}' non trouvée dans l'arbre.", r);
        }
    }
}
