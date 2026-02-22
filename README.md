# 🏺 Moteur de Recherche Morphologique Arabe

Implémentation d'un projet algorithmique complet pour un moteur morphologique arabe. Ce système utilise des structures de données avancées pour modéliser la nature Racine-Schème de la langue arabe, permettant la génération et la validation de mots dérivés.

## 📋 Aperçu du Projet

L'objectif de ce projet est de développer un outil capable d'indexer des racines arabes et de gérer des schèmes morphologiques pour générer des dérivés (ex : transformer "كتب" en "كاتب", "مكتوب"). Il met l'accent sur l'efficacité algorithmique par l'implémentation manuelle de structures de données fondamentales.

### Fonctionnalités Principales

- **Gestion des Racines :** Indexation des racines trilitères (ex: ك-ت-ب) à l'aide d'un Arbre Binaire de Recherche (ABR) pour une complexité de recherche logarithmique.
- **Système de Schèmes :** Accès rapide aux modèles morphologiques (ex: فاعل) via une Table de Hachage implémentée manuellement.
- **Moteur de Dérivation :** Génération dynamique de mots en fusionnant des racines avec des schèmes abstraits.
- **Validation :** Capacité de recherche inversée pour vérifier si un mot appartient à une famille de racines spécifique.
- **Interface CLI :** Une interface en ligne de commande complète pour manipuler les racines, les schèmes et les dérivés.

## 🛠 Architecture Technique

Le projet est développé en **Rust** pour garantir la sécurité mémoire et la performance, en respectant strictement les contraintes algorithmiques.

### 1. Structures de Données

#### 🌳 Arbre Binaire de Recherche (ABR) pour les Racines (`src/arbre.rs`)

- **But :** Stocker les racines arabes.
- **Structure :** Chaque nœud contient :
  - La racine de 3 lettres.
  - Une liste de mots dérivés validés (`Vec<Derive>`).
  - Un compteur de fréquence.
  - Des pointeurs vers les enfants gauche/droit.
- **Algorithme :** Un comparateur personnalisé pour les caractères arabes assure un ordre lexicographique correct.

#### ⚡ Table de Hachage pour les Schèmes (`src/hashing.rs`)

- **But :** Stocker les schèmes morphologiques abstraits.
- **Implémentation :**
  - **Implémentation Manuelle :** Pas d'utilisation de la HashMap standard.
  - **Résolution des Collisions :** Adressage ouvert avec **Double Hachage** (`hash1` et `hash2`) pour minimiser le clustering.
  - **Opérations :** Supporte l'Insertion, la Suppression (avec marqueurs "deleted"), et la Recherche.

### 2. Analyseur Morphologique (`src/morpho_analyzer.rs`)

La logique centrale repose sur le système de substitution standard de l'arabe :

- **Mécanisme :** Utilisation de la racine fictive **"ف-ع-ل"** (F-A-L).
- **Processus :**
  1.  Racine d'entrée : `k-t-b` (c1, c2, c3).
  2.  Schème : `m-a-f-u-u-l` (مفعول).
  3.  L'algorithme de substitution remplace 'ف' par c1, 'ع' par c2, 'ل' par c3.
  4.  Résultat : `m-a-k-t-u-u-b` (مكتوب).

## 🚀 Démarrage

### Prérequis

- [Rust & Cargo](https://www.rust-lang.org/tools/install) installés.

### Installation

```bash
git clone https://github.com/votre-nom/moteur-morphologique.git
cd moteur-morphologique
```

### Utilisation

Lancez l'application en mode release pour des performances optimales :

```bash
cargo run --release
```

### Menu Interactif

L'application lancera une interface terminal avec les options suivantes :

1.  **Opérations sur les Racines :** Charger depuis un fichier (`racines.txt`), Ajouter, Chercher, Supprimer.
2.  **Analyse :** Prévisualiser les familles morphologiques, Générer des dérivés.
3.  **Validation :** Vérifier si un mot est un dérivé valide d'une racine.
4.  **Gestion des Schèmes :** Ajouter ou modifier des modèles de génération.

## 📂 Structure du Projet

```text
moteur_morphologique/
├── racines.txt           # Jeu de données initial des racines arabes
├── src/
│   ├── main.rs           # Point d'entrée & Logique du Menu CLI
│   ├── arbre.rs          # Implémentation ABR pour les Racines
│   ├── hashing.rs        # Implémentation Table de Hachage pour les Schèmes
│   ├── morpho_analyzer.rs # Logique linguistique (Génération/Validation)
│   └── terminal_adapter.rs # Utilitaires pour l'affichage du texte arabe
└── Cargo.toml            # Dépendances Rust
```

## 🧠 Complexité Algorithmique

- **Recherche de Racine :** $O(\log N)$ en moyenne (ABR), où $N$ est le nombre de racines.
- **Accès aux Schèmes :** $O(1)$ amorti grâce à la table de hachage.
- **Génération :** $O(L)$ où $L$ est la longueur de la chaîne du schème.

## 📝 Licence

Ce projet a été développé dans le cadre d'un Mini-Projet d'Algorithmique.
