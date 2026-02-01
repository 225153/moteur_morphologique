mod arbre;
mod hash_table;

use arbre::Tree;
use hash_table::init_schemes;

fn main() {
    // Test de la table de hachage
    println!("=== Test Table de Hachage ===\n");

    let schemes = init_schemes();
    schemes.display();

    println!("\n=== Test Recherche ===");
    if let Some(scheme) = schemes.get("فاعل") {
        println!("Trouvé: {} - {}", scheme.nom, scheme.description);
        println!("Pattern: {}", scheme.pattern);
    }

    println!("\n=== Test Arbre ===");
    let mut tree = Tree::new();
    tree.insert(['ك', 'ت', 'ب']);
    tree.insert(['د', 'ر', 'س']);
    tree.insert(['ع', 'ل', 'م']);

    println!("Vérification racine كتب: {}", tree.verify(['ك', 'ت', 'ب']));
    println!("Vérification racine xyz: {}", tree.verify(['x', 'y', 'z']));
}
