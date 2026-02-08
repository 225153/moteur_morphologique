mod arbre;
mod hashing;
mod morpho_analyzer;

use arbre::Tree;
use hashing::SchemeTable;
use hashing::init_schemes;
use morpho_analyzer::afficher_derives_stockes;
use morpho_analyzer::afficher_validation;
use morpho_analyzer::generer_et_stocker;
use morpho_analyzer::valider_et_stocker;
use std::io;

// Liste des schèmes utilisés pour la dérivation et la validation
const SCHEMES: [(&str, &str); 6] = [
    ("فاعل", "participe actif"),
    ("مفعول", "participe passif"),
    ("فعل", "verbe forme I"),
    ("تفعيل", "masdar forme II"),
    ("افتعل", "verbe forme VIII"),
    ("استفعل", "verbe forme X"),
];

// Lire une ligne de texte depuis le clavier
fn lire_ligne() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Lire une racine de 3 caractères arabes depuis le clavier
// L'utilisateur tape : ك ت ب (séparés par des espaces)
fn lire_racine() -> Option<[char; 3]> {
    println!("Entrez la racine (3 lettres séparées par des espaces, ex: ك ت ب) :");
    let ligne = lire_ligne();
    let chars: Vec<char> = ligne.chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() == 3 {
        Some([chars[0], chars[1], chars[2]])
    } else {
        println!("Erreur: il faut exactement 3 caractères.");
        None
    }
}

// Afficher le menu principal
fn afficher_menu() {
    println!();
    println!("╔══════════════════════════════════════════╗");
    println!("║   Moteur Morphologique Arabe             ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  1. Charger racines depuis fichier       ║");
    println!("║  2. Ajouter une racine                   ║");
    println!("║  3. Chercher une racine                  ║");
    println!("║  4. Générer les dérivés d'une racine     ║");
    println!("║  5. Valider un mot                       ║");
    println!("║  6. Afficher les dérivés d'une racine    ║");
    println!("║  7. Afficher l'arbre des racines         ║");
    println!("║  8. Afficher les schèmes                 ║");
    println!("║  9. Quitter                              ║");
    println!("╚══════════════════════════════════════════╝");
    print!("Choix > ");
    // Forcer l'affichage immédiat du "Choix > "
    use std::io::Write;
    io::stdout().flush().unwrap();
}

fn main() {
    // Créer l'arbre (vide au départ)
    let mut arbre = Tree::new();

    // Créer la table de hachage avec les schèmes pré-chargés
    let table_schemes: SchemeTable = init_schemes();

    println!("Bienvenue dans le Moteur Morphologique Arabe !");

    // Boucle principale du menu
    loop {
        afficher_menu();
        let choix = lire_ligne();

        match choix.as_str() {
            // === 1. Charger racines depuis fichier ===
            "1" => {
                println!("Entrez le chemin du fichier (ex: racines.txt) :");
                let chemin = lire_ligne();
                arbre.charger_depuis_fichier(&chemin);
            }

            // === 2. Ajouter une racine manuellement ===
            "2" => {
                if let Some(racine) = lire_racine() {
                    arbre.insert(racine);
                    let r: String = racine.iter().collect();
                    println!("Racine '{}' ajoutée.", r);
                }
            }

            // === 3. Chercher une racine dans l'arbre ===
            "3" => {
                if let Some(racine) = lire_racine() {
                    let r: String = racine.iter().collect();
                    if arbre.verify(racine) {
                        println!("✓ La racine '{}' existe dans l'arbre.", r);
                    } else {
                        println!("✗ La racine '{}' n'existe pas dans l'arbre.", r);
                    }
                }
            }

            // === 4. Générer les dérivés d'une racine ===
            "4" => {
                if let Some(racine) = lire_racine() {
                    // Vérifier que la racine existe
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!("Racine '{}' non trouvée. Ajout automatique...", r);
                        arbre.insert(racine);
                    }
                    generer_et_stocker(&mut arbre, racine, &SCHEMES);
                    // Afficher les dérivés stockés
                    afficher_derives_stockes(&mut arbre, racine);
                }
            }

            // === 5. Valider un mot ===
            "5" => {
                println!("Entrez le mot à valider :");
                let mot = lire_ligne();

                if let Some(racine) = lire_racine() {
                    // Vérifier la racine dans l'arbre d'abord
                    if !arbre.verify(racine) {
                        let r: String = racine.iter().collect();
                        println!("Racine '{}' non trouvée. Ajout automatique...", r);
                        arbre.insert(racine);
                    }
                    let (trouve, scheme) = valider_et_stocker(&mut arbre, &mot, racine, &SCHEMES);
                    let r: String = racine.iter().collect();
                    if trouve {
                        println!("✓ OUI : '{}' appartient à la racine '{}'", mot, r);
                        println!("  Schème : {}", scheme.unwrap());
                    } else {
                        println!("✗ NON : '{}' n'appartient pas à la racine '{}'", mot, r);
                    }
                }
            }

            // === 6. Afficher les dérivés d'une racine ===
            "6" => {
                if let Some(racine) = lire_racine() {
                    afficher_derives_stockes(&mut arbre, racine);
                }
            }

            // === 7. Afficher l'arbre complet ===
            "7" => {
                arbre.afficher();
            }

            // === 8. Afficher les schèmes ===
            "8" => {
                table_schemes.display();
            }

            // === 9. Quitter ===
            "9" => {
                println!("Au revoir !");
                break;
            }

            // Choix invalide
            _ => {
                println!("Choix invalide. Tapez un nombre entre 1 et 9.");
            }
        }
    }
}
