// src/arbre.rs

// Représente un mot dérivé (ex : مكتوب, كاتب)
pub struct Derive {
    pub mot: String,
    pub schema: String,
}

// Représente un nœud de l’arbre binaire (une racine)
pub struct RacineNode {
    pub valeur: String,                 // racine arabe (كتب)
    pub derives: Vec<Derive>,           // mots dérivés
    pub left: Option<Box<RacineNode>>,  // fils gauche
    pub right: Option<Box<RacineNode>>, // fils droit
}

impl RacineNode {
    // Crée une nouvelle racine (sans enfants)
    pub fn new(valeur: String) -> Self {
        RacineNode {
            valeur,
            derives: Vec::new(),
            left: None,
            right: None,
        }
    }
}
