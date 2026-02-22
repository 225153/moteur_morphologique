pub struct Derive {
    pub mot: String,
    pub schema: String,
}

use std::fs;

pub struct RacineNode {
    pub racine: [char; 3],
    pub derives: Vec<Derive>,
    pub frequence: u32,
    pub left: Option<Box<RacineNode>>,
    pub right: Option<Box<RacineNode>>,
}

pub fn morphologic_cmp(tree_racine: [char; 3], racine: [char; 3]) -> i8 {
    let mut i = 0;
    while i < 3 {
        if tree_racine[i] == racine[i] {
            i = i + 1;
        } else {
            if tree_racine[i] > racine[i] {
                return -1;
            } else {
                return 1;
            }
        }
    }
    return 0;
}

pub struct Tree {
    pub racine: Option<Box<RacineNode>>,
}

impl RacineNode {
    pub fn new(racine: [char; 3]) -> Self {
        RacineNode {
            racine,
            derives: Vec::new(),
            frequence: 0,
            left: None,
            right: None,
        }
    }

    pub fn ajouter_derive(&mut self, mot: String, schema: String) {
        for d in &self.derives {
            if d.mot == mot {
                return;
            }
        }

        self.derives.push(Derive {
            mot: mot,
            schema: schema,
        });
        self.frequence = self.frequence + 1;
    }

    pub fn supprimer_derive(&mut self, mot: &str) -> bool {
        if let Some(pos) = self.derives.iter().position(|d| d.mot == mot) {
            self.derives.remove(pos);
            self.frequence = self.frequence - 1;
            return true;
        }
        false
    }

    pub fn afficher_derives(&self) {
        let r: String = self.racine.iter().collect();
        let r_display: String = r.chars().rev().collect();
        println!("Racine: {} ({} dérivés)", r_display, self.frequence);
        for d in &self.derives {
            let mot_display: String = d.mot.chars().rev().collect();
            let schema_display: String = d.schema.chars().rev().collect();
            println!("  - {} (schème: {})", mot_display, schema_display);
        }
    }

    pub fn afficher_in_order(&self) {
        if let Some(gauche) = &self.left {
            gauche.afficher_in_order();
        }

        let r: String = self.racine.iter().collect();
        let r_display: String = r.chars().rev().collect();
        if self.frequence > 0 {
            println!("  {} ({} dérivés)", r_display, self.frequence);
        } else {
            println!("  {}", r_display);
        }

        if let Some(droite) = &self.right {
            droite.afficher_in_order();
        }
    }

    pub fn verify_node(&self, ch: [char; 3]) -> bool {
        let cmp = morphologic_cmp(self.racine, ch);
        if cmp == 0 {
            return true;
        } else {
            if cmp == -1 {
                if self.left.is_none() {
                    return false;
                } else {
                    return self.left.as_ref().unwrap().verify_node(ch);
                }
            } else {
                if self.right.is_none() {
                    return false;
                } else {
                    return self.right.as_ref().unwrap().verify_node(ch);
                }
            }
        }
    }
    pub fn insert_node(&mut self, ch: [char; 3]) {
        let cmp = morphologic_cmp(self.racine, ch);
        if cmp == 1 {
            if self.right.is_none() {
                self.right = Some(Box::new(RacineNode::new(ch)));
            } else {
                self.right.as_mut().unwrap().insert_node(ch);
            }
        } else {
            if cmp == 0 {
                return;
            } else {
                if self.left.is_none() {
                    self.left = Some(Box::new(RacineNode::new(ch)));
                } else {
                    self.left.as_mut().unwrap().insert_node(ch);
                }
            }
        }
    }
}
impl Tree {
    pub fn new() -> Self {
        Tree { racine: None }
    }
    pub fn verify(&self, ch: [char; 3]) -> bool {
        if self.racine.is_none() {
            return false;
        }
        let node = self.racine.as_ref().unwrap();
        return node.verify_node(ch);
    }
    pub fn insert(&mut self, ch: [char; 3]) {
        if self.racine.is_none() {
            self.racine = Some(Box::new(RacineNode::new(ch)));
            return;
        }
        self.racine.as_mut().unwrap().insert_node(ch);
    }

    pub fn delete(&mut self, ch: [char; 3]) -> bool {
        fn delete_node(node: &mut Option<Box<RacineNode>>, ch: [char; 3]) -> bool {
            if let Some(mut current) = node.take() {
                let cmp = morphologic_cmp(current.racine, ch);

                if cmp == 0 {
                    *node = match (current.left.take(), current.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(right)) => {
                            let mut successor_parent = right;
                            if successor_parent.left.is_none() {
                                successor_parent.left = Some(left);
                                Some(successor_parent)
                            } else {
                                let mut parent = &mut successor_parent;
                                while parent.left.as_ref().unwrap().left.is_some() {
                                    parent = parent.left.as_mut().unwrap();
                                }
                                let mut successor = parent.left.take().unwrap();
                                parent.left = successor.right.take();
                                successor.left = Some(left);
                                successor.right = Some(successor_parent);
                                Some(successor)
                            }
                        }
                    };
                    return true;
                } else if cmp == -1 {
                    let found = delete_node(&mut current.left, ch);
                    *node = Some(current);
                    return found;
                } else {
                    let found = delete_node(&mut current.right, ch);
                    *node = Some(current);
                    return found;
                }
            }
            false
        }

        delete_node(&mut self.racine, ch)
    }

    pub fn chercher_noeud(&mut self, ch: [char; 3]) -> Option<&mut RacineNode> {
        let mut courant = self.racine.as_mut();

        while let Some(noeud) = courant {
            let cmp = morphologic_cmp(noeud.racine, ch);
            if cmp == 0 {
                return Some(noeud);
            } else if cmp == -1 {
                courant = noeud.left.as_mut();
            } else {
                courant = noeud.right.as_mut();
            }
        }
        None
    }

    pub fn ajouter_derive(&mut self, ch: [char; 3], mot: String, schema: String) -> bool {
        let noeud = self.chercher_noeud(ch);
        match noeud {
            Some(n) => {
                n.ajouter_derive(mot, schema);
                true
            }
            None => false,
        }
    }

    pub fn charger_depuis_fichier(&mut self, chemin: &str) -> u32 {
        let contenu = fs::read_to_string(chemin);

        let texte = match contenu {
            Ok(t) => t,
            Err(e) => {
                println!("Erreur lecture fichier '{}': {}", chemin, e);
                return 0;
            }
        };

        let mut compteur: u32 = 0;

        for ligne in texte.lines() {
            let ligne = ligne.trim();
            if ligne.is_empty() {
                continue;
            }

            let chars: Vec<char> = ligne.chars().filter(|c| !c.is_whitespace()).collect();

            if chars.len() == 3 {
                let racine: [char; 3] = [chars[0], chars[1], chars[2]];
                self.insert(racine);
                compteur = compteur + 1;
            } else {
                let ligne_display: String = ligne.chars().rev().collect();
                println!("Ligne ignorée (pas 3 caractères): '{}'", ligne_display);
            }
        }

        println!("{} racines chargées depuis '{}'", compteur, chemin);
        compteur
    }

    pub fn afficher(&self) {
        if self.racine.is_none() {
            println!("L'arbre est vide.");
            return;
        }
        println!("=== Racines stockées (ordre trié) ===");
        self.racine.as_ref().unwrap().afficher_in_order();
    }
}
