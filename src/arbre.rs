pub struct Derive {
    pub mot: String,
    pub schema: String,
}

pub struct RacineNode {
    pub racine: [char; 3],
    pub derive: Vec<Derive>,
    pub frequence: usize,
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
            derive: Vec::new(),
            frequence: 0,
            left: None,
            right: None,
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
    pub fn has_derive(&self, mot: &str) -> bool {
        self.derive.iter().any(|d| d.mot == mot)
    }
    pub fn add_derive(&mut self, mot: String, schema: String) {
        if !self.has_derive(&mot) {
            self.derive.push(Derive { mot, schema });
            self.frequence += 1;
        }
    }
    pub fn get_derives(&self) -> &Vec<Derive> {
        &self.derive
    }

    pub fn display_inorder(&self) {
        // Parcourir le sous-arbre gauche
        if let Some(left) = &self.left {
            left.display_inorder();
        }

        // Afficher le nœud actuel
        println!(
            "Racine: {}{}{}",
            self.racine[0], self.racine[1], self.racine[2]
        );
        println!("  Fréquence: {}", self.frequence);
        println!("  Dérivés:");
        for derive in &self.derive {
            println!("    - {} (schème: {})", derive.mot, derive.schema);
        }
        println!();

        // Parcourir le sous-arbre droit
        if let Some(right) = &self.right {
            right.display_inorder();
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
    pub fn display(&self) {
        if let Some(root) = &self.racine {
            root.display_inorder();
        } else {
            println!("L'arbre est vide");
        }
    }
}
