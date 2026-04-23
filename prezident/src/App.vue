<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import MarkdownIt from 'markdown-it';


onMounted(() => {
  const tabs = document.querySelectorAll<HTMLElement>('.tab');

  const sections: Record<string, HTMLElement | null> = {
    Config: document.getElementById('Config'),
    Presentation: document.getElementById('Prez'),
    Stylesheet: document.getElementById('Stylesheet'),
    Assets: document.getElementById('Assets'),
    Preview: document.getElementById('Preview'),
  };

  const hideAllSections = () => {
    Object.values(sections).forEach((section) => {
      if (section) section.style.display = 'none';
    });
  };

  hideAllSections();
  sections.Config?.style.setProperty('display', 'block');

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      tabs.forEach((t) => t.classList.remove('active'));
      tab.classList.add('active');

      hideAllSections();

      const key = tab.textContent?.trim();
      const targetSection = key ? sections[key] : null;

      if (targetSection) {
        targetSection.style.display = 'block';
      }

      console.log(`Onglet: ${key}`);
    });
  });
});

// markdown to html
const md = new MarkdownIt({
  html: true,        
  linkify: true,     
  typographer: true  
});

const markdownText = ref<string>('');

const slides = computed<string[]>(() =>
  markdownText.value
    .split(/^---$/gm) 
    .map(s => md.render(s.trim()))
);

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
      <textarea id="Prez" v-model="markdownText" rows="8" cols="50"></textarea>

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
      <div id="Preview" v-for="(slide, index) in slides" :key="index" v-html="slide"></div>

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

.workspace #Preview {
  width: 100%;
  background: white;
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 16px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.slide {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  padding: 20px;
  min-height: 300px;
  line-height: 1.6;
  font-size: 16px;
}

</style>