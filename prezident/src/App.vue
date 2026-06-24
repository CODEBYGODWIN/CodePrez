<script setup lang="ts">
import { computed, ref, onMounted } from 'vue';
import MarkdownIt from 'markdown-it';
import PresentationMode from './components/PresentationMode.vue';
import MarkdownStyle from 'markdown-it-style';
import './style/app.css';

import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';

const configContent = ref('{\n  "title": "",\n  "presenters": [],\n  "duration": 0\n}');
const markdownText = ref<string>('');
const stylesheetContent = ref('');
const isPresentationMode = ref(false);
const markdownStyle = ref<string>('');
const folderOpened = ref<string>('');  // dossier temp extrait (pour assets)
const codeprezPath = ref<string>('');  // chemin du .codeprez (pour re-save)
const pictures = ref<string[]>([]);

// Expression régulière pour trouver les images Markdown: ![alt](filename)
const pattern = /!\[(.*?)\]\(([^)\s]+)\)/g;

const CODEPREZ_FILTER = [{ name: 'CodePrez', extensions: ['codeprez'] }];

const prezTitle = computed(() => {
  if (!codeprezPath.value) return 'New presentation';
  return codeprezPath.value.split(/[\\/]/).pop()?.replace('.codeprez', '') ?? 'New presentation';
});

async function handleOpen() {
  const file = await open({
    filters: CODEPREZ_FILTER,
    title: 'Ouvrir une présentation',
  });

  if (!file) return;

  try {
    const data = await invoke<{ config: string; presentation: string; stylesheet: string; temp_folder: string }>('open_codeprez', {
      filePath: file as string,
    });
    configContent.value = data.config;
    markdownText.value = data.presentation;
    stylesheetContent.value = data.stylesheet;
    folderOpened.value = data.temp_folder;
    codeprezPath.value = file as string;

    pictures.value = await invoke<string[]>('list_files', {
      folderPath: data.temp_folder + '/assets',
    });
  } catch (err) {
    alert(`Erreur lors de l'ouverture :\n${err}`);
  }
}

async function handleSave() {
  let filePath = codeprezPath.value;

  if (!filePath) {
    const picked = await save({
      filters: CODEPREZ_FILTER,
      title: 'Enregistrer la présentation',
    });
    if (!picked) return;
    filePath = picked as string;
    codeprezPath.value = filePath;
  }

  const assetsFolder = folderOpened.value ? folderOpened.value + '/assets' : '';

  try {
    await invoke('save_codeprez', {
      filePath,
      config: configContent.value,
      presentation: markdownText.value,
      stylesheet: markdownStyle.value,
      assetsFolder,
    });
    alert(`Présentation sauvegardée dans :\n${filePath}`);
  } catch (err) {
    alert(`Erreur lors de la sauvegarde :\n${err}`);
  }
}

async function loadPicture() {
  if (!folderOpened.value) {
    folderOpened.value = await invoke<string>('create_temp_project');
  }

  const file = await open({
    title: 'Choisir une image',
    multiple: false,
    directory: false,
    filters: [
      { name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif'] },
    ],
  });

  if (!file || Array.isArray(file)) return;

  await invoke('copy_image_to_assets', {
    sourcePath: file,
    folderPath: folderOpened.value + '/assets',
  });
  
  try {
    const files = await invoke<string[]>('list_files', {
      folderPath: folderOpened.value + "/assets" as string,
    });

    pictures.value = files;
  }
  catch (err) {
    alert(`Erreur lors de l'ouverture :\n${err}`);
  }
}

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

  window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 's') { e.preventDefault(); handleSave(); }
    if (e.ctrlKey && e.key === 'o') { e.preventDefault(); handleOpen(); }
  });

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
    });
  });
});

const fullScreen = () => {
  if (slides.value.length > 0) {
    isPresentationMode.value = true;
  } else {
    alert('Aucune slide trouvée. Veuillez ajouter du contenu dans la section Présentation.');
  }
};

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true
});

function cssToObject(cssString:string) {
  const result: Record<string, string> = {};
  // Supprime les espaces superflus autour des accolades et découpe les blocs
  const blocks = cssString
    .replace(/\s*{\s*/g, '{')
    .replace(/\s*}\s*/g, '}')
    .trim()
    .match(/([^{}]+)\{([^{}]*)\}/g);

  if (!blocks) return result;

  for (const block of blocks) {
    const match = block.match(/([^{}]+)\{([^{}]*)\}/);
    if (!match) continue;

    const selector = match[1].trim();
    let styles = match[2].trim();

    // Supprime les points-virgules en fin de chaîne et partout pour uniformiser
    styles = styles.replace(/;+$/, '').replace(/;\s*/g, ';');

    result[selector] = styles;
  }

  return result;
}

const slides = computed<string[]>(() =>{
  var obj = cssToObject(markdownStyle.value);
  md.use(MarkdownStyle, obj);
  var cheminPrefixe = folderOpened.value+"\\assets\\";
  const resultat = markdownText.value.replace(pattern, (match, altText, currentFilename) => {
  // Vérifier si le chemin est déjà présent
  const adejaChemin = 
    currentFilename.startsWith("/") || 
    currentFilename.startsWith("./") || 
    currentFilename.startsWith("../") ||
    currentFilename.startsWith("http://") || 
    currentFilename.startsWith("https://");
  
  if (adejaChemin) {
    // Chemin déjà présent, ne rien modifier
    return match;
  }
  
  // Ajouter le préfixe de chemin
  const newPath = convertFileSrc(`${cheminPrefixe}${currentFilename}`);
  return `![${altText}](${newPath})`;
  });

  const slideList = resultat.split(/^---$/gm).map(s => md.render(s.trim()));

  // Diapositive de titre générée depuis config.json
  try {
    const config = JSON.parse(configContent.value);
    const presenters = Array.isArray(config.presenters) ? config.presenters.join(', ') : '';
    slideList.unshift(`<h1>${config.title ?? ''}</h1>${presenters ? `<p>${presenters}</p>` : ''}`);
  } catch { /* config.json invalide, on ignore */ }

  return slideList;
},
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
  <PresentationMode
    v-if="isPresentationMode"
    :slides="slides"
    @close="isPresentationMode = false"
  />

  <div class="app">
    <header class="topbar">
      <div class="topbar-left">
        <button class="btn btn-primary" @click="handleSave">Save</button>
        <button class="btn btn-primary" @click="handleOpen">Open</button>
      </div>
      <div class="topbar-title" id="PrezName">{{ prezTitle }}</div>
      <div class="topbar-right">
        <button class="btn btn-primary" @click="fullScreen">Presentation</button>
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
      <textarea id="Config" v-model="configContent"></textarea>
      <textarea id="Prez" v-model="markdownText" rows="8" cols="50"></textarea>
      <textarea id="Stylesheet" v-model="markdownStyle"></textarea>

      <div id="Assets">
        <button id="AddAsset" class="btn btn-primary" @click="loadPicture">Add +</button>
        <ul class="asset-list">
          <li v-for="(picture, index) in pictures" :key="index">{{ picture }}</li>
        </ul>
      </div>

      <div id="Preview">
        <div v-for="(slide, index) in slides" :key="index" v-html="slide"></div>
      </div>
    </main>
  </div>
</body>
</html>
</template>