# Guide de Migration vers Web/Mobile

## Architecture Actuelle (Terminal)

Le code est organisé avec **un module spécial pour le terminal Windows** qui gère les problèmes d'affichage RTL arabe.

### Fichiers du projet :

```
src/
├── main.rs              # CLI interface (À ADAPTER pour API web)
├── terminal_adapter.rs  # ⚠️ À SUPPRIMER pour le web ⚠️
├── arbre.rs            # ✅ Garder tel quel (logique métier)
├── hashing.rs          # ✅ Garder tel quel (logique métier)
└── morpho_analyzer.rs  # ✅ Garder tel quel (logique métier)
```

## 📋 Checklist complète pour la migration Web/Mobile

### Étape 1 : Supprimer le module terminal

```rust
// Dans main.rs - SUPPRIMER ces lignes :
mod terminal_adapter;  // ← Supprimer
use terminal_adapter::{afficher_arabe, lire_ligne_simple, lire_racine_terminal, lire_texte_arabe};  // ← Supprimer
use std::io;  // ← Supprimer (plus nécessaire)
```

```bash
# Supprimer le fichier
rm src/terminal_adapter.rs
```

### Étape 2 : Transformer en API REST (exemple avec Actix-web)

**Avant (Terminal CLI) :**

```rust
fn main() {
    let mut arbre = Tree::new();
    let mut table_schemes = init_schemes();

    // Boucle menu terminal...
    if let Some(racine) = lire_racine_terminal() {
        arbre.insert(racine);
        let r: String = racine.iter().collect();
        println!("Racine '{}' ajoutée.", afficher_arabe(&r));
    }
}
```

**Après (API Web) :**

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RacineInput {
    caracteres: [char; 3]  // Reçu directement en RTL du frontend
}

#[derive(Serialize)]
struct RacineResponse {
    message: String,
    racine: String
}

#[post("/racines")]
async fn ajouter_racine(
    input: web::Json<RacineInput>,
    arbre: web::Data<Mutex<Tree>>
) -> HttpResponse {
    let mut arbre = arbre.lock().unwrap();
    arbre.insert(input.caracteres);

    let r: String = input.caracteres.iter().collect();
    // PAS DE afficher_arabe() - le frontend gère le RTL !
    HttpResponse::Ok().json(RacineResponse {
        message: "Racine ajoutée".to_string(),
        racine: r  // Envoi direct en RTL
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arbre = web::Data::new(Mutex::new(Tree::new()));
    let schemes = web::Data::new(Mutex::new(init_schemes()));

    HttpServer::new(move || {
        App::new()
            .app_data(arbre.clone())
            .app_data(schemes.clone())
            .service(ajouter_racine)
            // ... autres routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Étape 3 : Frontend HTML avec support RTL natif

```html
<!DOCTYPE html>
<html lang="ar" dir="rtl">
  <head>
    <meta charset="UTF-8" />
    <title>محرك التصريف المورفولوجي</title>
    <style>
      body {
        direction: rtl;
        text-align: right;
        font-family: "Amiri", "Arial", sans-serif;
      }
      .arabe {
        direction: rtl;
        unicode-bidi: embed;
      }
    </style>
  </head>
  <body>
    <h1>أضف جذر</h1>
    <input type="text" id="racine" class="arabe" placeholder="ك ت ب" />
    <button onclick="ajouterRacine()">إضافة</button>

    <div id="resultat" class="arabe"></div>

    <script>
      async function ajouterRacine() {
        const input = document.getElementById("racine").value;
        const chars = input.split(" ").filter((c) => c.trim());

        // Les caractères sont déjà en RTL - pas de transformation !
        const response = await fetch("/racines", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ caracteres: chars }),
        });

        const data = await response.json();
        // Le navigateur affiche automatiquement le RTL correctement
        document.getElementById("resultat").textContent =
          `✓ ${data.message}: ${data.racine}`;
      }
    </script>
  </body>
</html>
```

## 🎯 Différences Clés : Terminal vs Web

| Aspect        | Terminal Windows                               | Web/Mobile                            |
| ------------- | ---------------------------------------------- | ------------------------------------- |
| **Stockage**  | RTL (ك ت ب)                                    | RTL (ك ت ب) ✅ Identique              |
| **Affichage** | Inverse avec `.rev()` car terminal LTR         | `dir="rtl"` natif - PAS d'inversion   |
| **Input**     | `lire_racine_terminal()` lit et NE inverse PAS | Input HTML direct en RTL              |
| **Output**    | `afficher_arabe()` inverse chaque String       | Affichage direct - le navigateur gère |

## ⚠️ IMPORTANT : Ce qui change, ce qui NE change PAS

### ✅ À GARDER tel quel (logique métier) :

- `arbre.rs` : Toutes les fonctions (insert, delete, verify, etc.)
- `hashing.rs` : Table de hachage complète
- `morpho_analyzer.rs` : `generer_mot()`, `valider_mot()`, etc.
- **Stockage RTL** : Les données sont DÉJÀ dans le bon format !

### ❌ À SUPPRIMER (spécifique terminal) :

- Fichier `terminal_adapter.rs` entier
- Toutes les fonctions `afficher_arabe()`
- Toutes les fonctions `lire_*_terminal()`
- Les inversions `.rev()` d'affichage dans `arbre.rs`, `morpho_analyzer.rs`, `hashing.rs`

### 🔄 À REMPLACER (interface) :

- `main.rs` : Remplacer la boucle CLI par des routes API
- Les `println!` par des `HttpResponse::Ok().json()`
- Les `lire_*` par des paramètres de requête HTTP

## 📱 Mobile (React Native / Flutter)

Le principe est identique au web :

```javascript
// React Native
<Text style={{ writingDirection: 'rtl' }}>
  {racine}  {/* Affiché automatiquement RTL */}
</Text>

// Flutter
Text(
  racine,
  textDirection: TextDirection.rtl,
)
```

## 🧪 Test de compatibilité actuelle

Le code actuel stocke déjà tout en RTL. Vous pouvez le vérifier :

```rust
// Dans le terminal actuel :
// Ajoutez une racine: ك ت ب
// Regardez racines.txt : les données sont en RTL ✓

// Dans le futur web :
// Même donnée reçue en RTL
// Affichée en RTL avec dir="rtl"
// AUCUNE transformation nécessaire !
```

## ✨ Résumé

**Le code est DÉJÀ prêt pour le web** car :

1. Les données sont stockées en RTL (format correct)
2. La logique métier ne dépend PAS de l'affichage
3. Seul le module `terminal_adapter.rs` est spécifique au terminal
4. Supprimez ce module → tout fonctionne pour le web !

**Migration =**

- ✅ Garder 95% du code (arbre, hashing, morpho_analyzer)
- ❌ Supprimer 1 fichier (terminal_adapter.rs)
- 🔄 Adapter 1 fichier (main.rs) pour API REST au lieu de CLI

Le gros du travail (algorithmes, structures de données) est fait ! 🚀
