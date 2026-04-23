<script setup lang="ts">
document.addEventListener('DOMContentLoaded', () => {
    const tabs = document.querySelectorAll('.tab');
    const sections = {
        'Config': document.getElementById('Config'),
        'Presentation': document.getElementById('Prez'),
        'Stylesheet': document.getElementById('Stylesheet'),
        'Assets': document.getElementById('Assets'),
        'Preview': document.getElementById('Preview')
    };

    // Masquer toutes les sections au démarrage
    Object.values(sections).forEach(section => {
        if (section) {
            section.style.display = 'none';
        }
    });

    // Afficher la section Config par défaut
    if (sections.Config) sections.Config.style.display = 'block';

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            // 1. Changer onglet actif
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');

            // 2. Masquer toutes les sections
            Object.values(sections).forEach(section => {
                if (section) {
                    section.style.display = 'none';
                }
            });

            // 3. Afficher la section correspondante
            const targetSection = sections[tab.textContent];
            if (targetSection) {
                targetSection.style.display = 'block';
            }

            console.log(`Onglet: ${tab.textContent}`);
        });
    });
});
</script>

<template>
<html lang="fr">
<head>
  <meta charset="UTF-8" />
  <title>non prez</title>
  <link rel="stylesheet" href="/src/assets/base.css" />
</head>
<body>
  <div class="app">
    <header class="topbar">
      <div class="topbar-left">
        <button class="btn btn-primary">New prez</button>
        <button class="btn btn-primary">Save</button>
        <button class="btn btn-primary">Open</button>
      </div>
      <div class="topbar-title" id="PrezName">non prez</div>
      <div class="topbar-right">
        <button class="btn btn-primary">Presentation</button>
      </div>
    </header>

    <nav class="tabs">
      <button class="tab active">Config</button>
      <button class="tab">Presentation</button>
      <button class="tab">Stylesheet</button>
      <button class="tab">Assets</button>
      <button class="tab">Preview</button>
    </nav>

    <main class="workspace">
      <!-- Elements visible en Config -->
      <textarea id="Config"></textarea>

      <!-- Elements visible en Presentation -->
      <textarea id="Prez"></textarea>

      <!-- Elements visible en Stylesheet -->
      <textarea id="Stylesheet"></textarea>

      <!-- Elements visible en Assets -->
      <div id="Assets">
        <button id="AddAsset" class="btn btn-primary">Add +</button>
        <li>
          <ul> image.png </ul>
          <ul> image2.jpg </ul>
          <ul> code.js </ul>
          <ul> code2.ts </ul>
        </li>
      </div>
      <!-- temporaire faire un template generé avec la listes des fichiers en assets -->

      <!-- Elements visible en Preview -->
      <!-- a remplir avec le template d'affichage -->
    </main>
  </div>
</body>
</html>
</template>

<style scoped>
</style>

<style>
/* styles.css */

*,
*::before,
*::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI",
    sans-serif;
  background: #ffffff;
}

.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Barre supérieure */

.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px;
}

.topbar-left,
.topbar-right {
  display: flex;
  gap: 8px;
}

.topbar-title {
  font-size: 14px;
  color: #333;
}

/* Boutons */

.btn {
  border: none;
  border-radius: 4px;
  padding: 8px 14px;
  font-size: 13px;
  cursor: pointer;
  background: #333;
  color: #fff;
}

.btn-primary:hover {
  background: #111;
}

/* Onglets */

.tabs {
  display: flex;
  gap: 16px;
  padding: 0 16px;
  border-bottom: 1px solid #ddd;
  font-size: 13px;
}

.tab {
  border: none;
  background: transparent;
  padding: 8px 0;
  cursor: pointer;
  color: #555;
}

.tab.active {
  color: #000;
  border-bottom: 2px solid #000;
}

/* Zone principale grise */

.workspace {
  flex: 1;
  margin: 8px 16px 16px;
  background: #e0e0e0;
}
/* Zone principale grise + contenus */
.workspace {
  flex: 1;
  margin: 8px 16px 16px;
  background: #e0e0e0;
  padding: 16px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  min-height: 0; /* Important pour flex */
}

/* Textareas */
.workspace textarea {
  width: 100%;
  height: 100%;
  min-height: 400px;
  resize: none; /* Désactive redimensionnement manuel */
  padding: 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  background: white;
  box-sizing: border-box;
}

/* Liste Assets */
.workspace #Assets {
  width: 100%;
  padding: 12px;
  background: white;
  border-radius: 4px;
  border: 1px solid #ddd;
  list-style: none;
}

.workspace #Assets ul {
  padding: 8px 12px;
  margin: 4px 0;
  background: #f8f9fa;
  border-radius: 4px;
  border-left: 3px solid #007bff;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 13px;
}

/* Preview (placeholder) */
.workspace #Preview {
  width: 100%;
  height: 500px;
  background: white;
  border: 2px dashed #ccc;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  font-size: 16px;
}

</style>