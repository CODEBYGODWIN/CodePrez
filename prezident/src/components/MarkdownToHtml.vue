<script setup lang="ts">
import { ref, computed } from 'vue';
import MarkdownIt from 'markdown-it';
import MarkdownStyle from 'markdown-it-style';

const md = new MarkdownIt({
  html: true,        
  linkify: true,     
  typographer: true  
});

const markdownText = ref<string>('');
const markdownStyle = ref<string>('');

const renderedHtml = computed<string>(() => {
  md.use(MarkdownStyle, markdownStyle);
  return md.render(markdownText.value);
});

</script>

<template>
  <div>
    <h2>Éditeur Markdown</h2>
    <textarea v-model="markdownText" rows="8" cols="50"></textarea>
    <textarea v-model="markdownStyle" rows="8" cols="50"></textarea>

    <h2>Résultat HTML</h2>
    <div v-html="renderedHtml"></div>
  </div>

</template>

<style scoped>
textarea {
  width: 100%;
  font-family: monospace;
}
</style>