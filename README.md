# CodePrez

CodePrez est une application de bureau pour créer et présenter des diaporamas écrits en Markdown. Le contenu, le style et les images sont regroupés dans un seul fichier `.codeprez` portable, et l'application offre un mode présentation plein écran.

Construite avec Tauri 2, Vue 3, TypeScript et Vite.

## Fonctionnalités

- Rédaction des slides en Markdown, séparées par `---`
- Slide de titre générée automatiquement à partir de `config.json` (titre + présentateurs)
- Style personnalisé via une feuille CSS appliquée au rendu Markdown
- Gestion des images : import dans le projet et référencement par simple nom de fichier
- Aperçu en direct du rendu
- Mode présentation plein écran avec navigation clavier (flèches) et molette
- Format `.codeprez` autonome : une archive ZIP contenant tout le nécessaire
- Raccourcis clavier : `Ctrl+S` pour sauvegarder, `Ctrl+O` pour ouvrir

## Le format `.codeprez`

Un fichier `.codeprez` est une archive ZIP avec la structure suivante :

```
config.json        # titre, présentateurs, durée
presentation.md    # contenu des slides en Markdown
style.css          # feuille de style appliquée au rendu
assets/            # images de la présentation
env/               # (réservé)
```

Exemple de `config.json` :

```json
{
  "title": "Ma présentation",
  "presenters": ["Alice", "Bob"],
  "duration": 0
}
```

## Prérequis

- [Node.js](https://nodejs.org/) (LTS recommandé)
- [Rust](https://www.rust-lang.org/tools/install) et Cargo
- Les [dépendances système de Tauri](https://tauri.app/start/prerequisites/) pour votre OS

## Installation

```bash
cd prezident
npm install
```

## Développement

Lance l'application en mode développement avec rechargement à chaud :

```bash
npm run tauri dev
```

Pour lancer uniquement le front-end Vite (sans la fenêtre Tauri) :

```bash
npm run dev
```

## Build

Génère un exécutable de production pour votre plateforme :

```bash
npm run tauri build
```

Les binaires sont produits dans `src-tauri/target/release/`.

## Utilisation

1. Ouvrez l'application.
2. Onglet **Config** : renseignez le titre et les présentateurs (JSON).
3. Onglet **Presentation** : rédigez vos slides en Markdown, séparées par une ligne `---`.
4. Onglet **Stylesheet** : ajustez le CSS appliqué au rendu.
5. Onglet **Assets** : importez vos images avec **Add +**, puis insérez-les en Markdown avec `![texte](nom-du-fichier.png)`.
6. Onglet **Preview** : vérifiez le rendu.
7. Cliquez sur **Presentation** pour lancer le plein écran. Naviguez avec les flèches ou la molette ; quittez avec `Échap`.
8. **Save** enregistre le tout dans un fichier `.codeprez`.

## Structure du projet

```
prezident/
├── src/                      # front-end Vue + TypeScript
│   ├── App.vue               # composant principal (éditeur, onglets, rendu Markdown)
│   ├── components/
│   │   └── PresentationMode.vue   # mode présentation plein écran
│   ├── style/
│   ├── types/
│   └── main.ts
├── src-tauri/                # back-end Rust (Tauri)
│   ├── src/lib.rs            # commandes : open/save .codeprez, gestion assets
│   └── tauri.conf.json
├── public/
└── package.json
```

## Stack technique

- **Tauri 2** — application de bureau et accès système (lecture/écriture de fichiers, archives ZIP)
- **Vue 3** + **TypeScript** — interface
- **Vite** — build et serveur de développement
- **markdown-it** + **markdown-it-style** — rendu du Markdown et application des styles
