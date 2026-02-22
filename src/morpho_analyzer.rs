pub fn generer_mot(racine: [char; 3], scheme_nom: &str) -> String {
    let mut resultat = String::new();

    for c in scheme_nom.chars() {
        if c == 'ف' {
            resultat.push(racine[0]);
        } else if c == 'ع' {
            resultat.push(racine[1]);
        } else if c == 'ل' {
            resultat.push(racine[2]);
        } else {
            resultat.push(c);
        }
    }

    resultat
}

pub fn generer_famille(racine: [char; 3], table: &SchemeTable) -> Vec<(String, String)> {
    let mut resultats: Vec<(String, String)> = Vec::new();

    for scheme in table.get_all_schemes() {
        let mot = generer_mot(racine, &scheme.nom);
        resultats.push((scheme.nom.clone(), mot));
    }

    resultats
}

pub fn afficher_famille(racine: [char; 3], table: &SchemeTable) {
    let r: String = racine.iter().collect();
    let r_display: String = r.chars().rev().collect();
    println!("=== Famille morphologique de {} ===", r_display);

    let famille = generer_famille(racine, table);

    for (scheme, mot) in &famille {
        let scheme_display: String = scheme.chars().rev().collect();
        let mot_display: String = mot.chars().rev().collect();
        println!("  {} → {}", scheme_display, mot_display);
    }

    println!("Total: {} dérivés", famille.len());
}

pub fn valider_mot(mot: &str, racine: [char; 3], table: &SchemeTable) -> (bool, Option<String>) {
    for scheme in table.get_all_schemes() {
        let mot_genere = generer_mot(racine, &scheme.nom);

        if mot_genere == mot {
            return (true, Some(scheme.nom.clone()));
        }
    }

    (false, None)
}

#[allow(dead_code)]
pub fn afficher_validation(mot: &str, racine: [char; 3], table: &SchemeTable) {
    let r: String = racine.iter().collect();
    let (trouve, scheme) = valider_mot(mot, racine, table);

    if trouve {
        println!("✓ OUI : '{}' appartient à la racine '{}'", mot, r);
        println!("  Schème utilisé : {}", scheme.unwrap());
    } else {
        println!("✗ NON : '{}' n'appartient pas à la racine '{}'", mot, r);
    }
}

use crate::arbre::Tree;
use crate::hashing::SchemeTable;

pub fn generer_et_stocker(arbre: &mut Tree, racine: [char; 3], table: &SchemeTable) -> u32 {
    let mut compteur: u32 = 0;

    for scheme in table.get_all_schemes() {
        let mot = generer_mot(racine, &scheme.nom);

        let ok = arbre.ajouter_derive(racine, mot.clone(), scheme.nom.clone());
        if ok {
            compteur = compteur + 1;
        }
    }

    let r: String = racine.iter().collect();
    let r_display: String = r.chars().rev().collect();
    println!(
        "{} dérivés générés et stockés pour la racine '{}'",
        compteur, r_display
    );
    compteur
}

pub fn valider_et_stocker(
    arbre: &mut Tree,
    mot: &str,
    racine: [char; 3],
    table: &SchemeTable,
) -> (bool, Option<String>) {
    let (trouve, scheme) = valider_mot(mot, racine, table);

    if trouve {
        let schema = scheme.clone().unwrap();
        arbre.ajouter_derive(racine, mot.to_string(), schema);
    }

    (trouve, scheme)
}

pub fn afficher_derives_stockes(arbre: &mut Tree, racine: [char; 3]) {
    let noeud = arbre.chercher_noeud(racine);
    match noeud {
        Some(n) => n.afficher_derives(),
        None => {
            let r: String = racine.iter().collect();
            let r_display: String = r.chars().rev().collect();
            println!("Racine '{}' non trouvée dans l'arbre.", r_display);
        }
    }
}
