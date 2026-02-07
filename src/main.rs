mod arbre;
mod hashing;
mod morpho_analyzer;

use arbre::Tree;
use morpho_analyzer::afficher_famille;
use morpho_analyzer::afficher_validation;
use morpho_analyzer::generer_mot;

fn main() {
    // === Test rapide du moteur de dérivation ===

    // Racine : ك ت ب
    let racine = ['ك', 'ت', 'ب'];

    // Test 1 : générer un seul mot
    let mot = generer_mot(racine, "فاعل");
    println!("Racine: ك ت ب + Schème: فاعل → {}", mot);

    let mot2 = generer_mot(racine, "مفعول");
    println!("Racine: ك ت ب + Schème: مفعول → {}", mot2);

    // Test 2 : générer toute la famille
    let schemes = [
        ("فاعل", "participe actif"),
        ("مفعول", "participe passif"),
        ("فعل", "verbe forme I"),
        ("تفعيل", "masdar forme II"),
        ("افتعل", "verbe forme VIII"),
        ("استفعل", "verbe forme X"),
    ];

    println!();
    afficher_famille(racine, &schemes);

    // Test 3 : une autre racine
    println!();
    afficher_famille(['د', 'ر', 'س'], &schemes);

    // === Test de la validation morphologique ===
    println!();
    println!("=== Validation morphologique ===");

    // Test 4 : مكتوب appartient à ك ت ب ?
    afficher_validation("مكتوب", racine, &schemes);

    // Test 5 : كاتب appartient à ك ت ب ?
    afficher_validation("كاتب", racine, &schemes);

    // Test 6 : تدريس appartient à د ر س ?
    afficher_validation("تدريس", ['د', 'ر', 'س'], &schemes);

    // Test 7 : mot qui n'appartient pas
    afficher_validation("سلام", racine, &schemes);
}
