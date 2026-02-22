#[derive(Clone)]
pub struct Scheme {
    pub nom: String,
    #[allow(dead_code)]
    pub pattern: String,
    pub description: String,
}

#[derive(Clone)]
enum Slot {
    Empty,
    Deleted,
    Occupied(String, Scheme),
}

pub struct SchemeTable {
    table: Vec<Slot>,
    size: usize,
}

impl SchemeTable {
    pub fn new(size: usize) -> Self {
        SchemeTable {
            table: vec![Slot::Empty; size],
            size,
        }
    }

    fn hash1(&self, key: &str) -> usize {
        let mut hash = 0;
        for ch in key.chars() {
            hash = hash * 31 + (ch as usize);
        }
        hash % self.size
    }

    fn hash2(&self, key: &str) -> usize {
        let mut hash = 0;
        for ch in key.chars() {
            hash = hash * 37 + (ch as usize);
        }
        let step = hash % (self.size - 1);
        if step == 0 { 1 } else { step }
    }

    pub fn insert(&mut self, key: String, scheme: Scheme) {
        let mut index = self.hash1(&key);
        let step = self.hash2(&key);
        let mut first_deleted: Option<usize> = None;

        for _ in 0..self.size {
            match &self.table[index] {
                Slot::Empty => {
                    let pos = first_deleted.unwrap_or(index);
                    self.table[pos] = Slot::Occupied(key, scheme);
                    return;
                }
                Slot::Deleted => {
                    if first_deleted.is_none() {
                        first_deleted = Some(index);
                    }
                    index = (index + step) % self.size;
                }
                Slot::Occupied(existing_key, _) if existing_key == &key => {
                    self.table[index] = Slot::Occupied(key, scheme);
                    return;
                }
                _ => {
                    index = (index + step) % self.size;
                }
            }
        }

        if let Some(pos) = first_deleted {
            self.table[pos] = Slot::Occupied(key, scheme);
            return;
        }

        panic!("Table pleine ! Augmentez la taille.");
    }

    pub fn get(&self, key: &str) -> Option<&Scheme> {
        let mut index = self.hash1(key);
        let step = self.hash2(key);

        for _ in 0..self.size {
            match &self.table[index] {
                Slot::Empty => return None,
                Slot::Deleted => {
                    index = (index + step) % self.size;
                }
                Slot::Occupied(existing_key, scheme) => {
                    if existing_key == key {
                        return Some(scheme);
                    }

                    index = (index + step) % self.size;
                }
            }
        }

        None
    }

    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn display(&self) {
        println!("=== Schèmes Morphologiques ===");
        let mut count = 0;
        for (i, slot) in self.table.iter().enumerate() {
            if let Slot::Occupied(key, scheme) = slot {
                let key_display: String = key.chars().rev().collect();
                let nom_display: String = scheme.nom.chars().rev().collect();
                println!(
                    "[{}] {} → {} ({})",
                    i, key_display, nom_display, scheme.description
                );
                count += 1;
            }
        }
        println!("Total: {} schèmes", count);
    }

    pub fn delete(&mut self, key: &str) -> bool {
        let mut index = self.hash1(key);
        let step = self.hash2(key);

        for _ in 0..self.size {
            match &self.table[index] {
                Slot::Empty => return false,
                Slot::Deleted => {
                    index = (index + step) % self.size;
                }
                Slot::Occupied(existing_key, _) => {
                    if existing_key == key {
                        self.table[index] = Slot::Deleted;

                        return true;
                    }

                    index = (index + step) % self.size;
                }
            }
        }
        false
    }

    pub fn get_all_schemes(&self) -> Vec<&Scheme> {
        self.table
            .iter()
            .filter_map(|slot| {
                if let Slot::Occupied(_, scheme) = slot {
                    Some(scheme)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn init_schemes() -> SchemeTable {
    let mut table = SchemeTable::new(31);

    table.insert(
        "فاعل".to_string(),
        Scheme {
            nom: "فاعل".to_string(),
            pattern: "ف-ا-ع-ل".to_string(),
            description: "participe actif".to_string(),
        },
    );

    table.insert(
        "مفعول".to_string(),
        Scheme {
            nom: "مفعول".to_string(),
            pattern: "م-ف-ع-و-ل".to_string(),
            description: "participe passif".to_string(),
        },
    );

    table.insert(
        "فعل".to_string(),
        Scheme {
            nom: "فعل".to_string(),
            pattern: "ف-ع-ل".to_string(),
            description: "verbe forme I".to_string(),
        },
    );

    // Forme II
    table.insert(
        "فعّل".to_string(),
        Scheme {
            nom: "فعّل".to_string(),
            pattern: "ف-ع-ّ-ل".to_string(),
            description: "verbe forme II".to_string(),
        },
    );

    table.insert(
        "تفعيل".to_string(),
        Scheme {
            nom: "تفعيل".to_string(),
            pattern: "ت-ف-ع-ي-ل".to_string(),
            description: "masdar forme II".to_string(),
        },
    );

    table.insert(
        "مفعّل".to_string(),
        Scheme {
            nom: "مفعّل".to_string(),
            pattern: "م-ف-ع-ّ-ل".to_string(),
            description: "participe actif forme II".to_string(),
        },
    );

    table.insert(
        "فاعل_III".to_string(),
        Scheme {
            nom: "فاعل".to_string(),
            pattern: "ف-ا-ع-ل".to_string(),
            description: "verbe forme III".to_string(),
        },
    );

    table.insert(
        "مفاعلة".to_string(),
        Scheme {
            nom: "مفاعلة".to_string(),
            pattern: "م-ف-ا-ع-ل-ة".to_string(),
            description: "masdar forme III".to_string(),
        },
    );

    table.insert(
        "أفعل".to_string(),
        Scheme {
            nom: "أفعل".to_string(),
            pattern: "أ-ف-ع-ل".to_string(),
            description: "verbe forme IV".to_string(),
        },
    );

    table.insert(
        "إفعال".to_string(),
        Scheme {
            nom: "إفعال".to_string(),
            pattern: "إ-ف-ع-ا-ل".to_string(),
            description: "masdar forme IV".to_string(),
        },
    );

    table.insert(
        "تفعّل".to_string(),
        Scheme {
            nom: "تفعّل".to_string(),
            pattern: "ت-ف-ع-ّ-ل".to_string(),
            description: "verbe forme V".to_string(),
        },
    );

    table.insert(
        "افتعل".to_string(),
        Scheme {
            nom: "افتعل".to_string(),
            pattern: "ا-ف-ت-ع-ل".to_string(),
            description: "verbe forme VIII".to_string(),
        },
    );

    table.insert(
        "مفتعل".to_string(),
        Scheme {
            nom: "مفتعل".to_string(),
            pattern: "م-ف-ت-ع-ل".to_string(),
            description: "participe forme VIII".to_string(),
        },
    );

    table.insert(
        "استفعل".to_string(),
        Scheme {
            nom: "استفعل".to_string(),
            pattern: "ا-س-ت-ف-ع-ل".to_string(),
            description: "verbe forme X".to_string(),
        },
    );

    table.insert(
        "مستفعل".to_string(),
        Scheme {
            nom: "مستفعل".to_string(),
            pattern: "م-س-ت-ف-ع-ل".to_string(),
            description: "participe forme X".to_string(),
        },
    );

    table
}
