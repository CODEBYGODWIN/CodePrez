<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const props = defineProps<{
  slides: string[];
}>();

const emit = defineEmits<{
  close: [];
}>();

const currentSlideIndex = ref(0);

const nextSlide = () => {
  if (currentSlideIndex.value < props.slides.length - 1) {
    currentSlideIndex.value++;
  }
};

const previousSlide = () => {
  if (currentSlideIndex.value > 0) {
    currentSlideIndex.value--;
  }
};

// Gestion des raccourcis clavier
const handleKeydown = (event: KeyboardEvent) => {
  switch (event.key) {
    case 'ArrowRight':
    case 'ArrowDown':
      nextSlide();
      break;
    case 'ArrowLeft':
    case 'ArrowUp':
      previousSlide();
      break;
    case 'Escape':
      emit('close');
      break;
  }
};

// Gestion du scroll pour changer de slide
const handleScroll = (event: WheelEvent) => {
  if (event.deltaY > 0) {
    nextSlide();
  } else if (event.deltaY < 0) {
    previousSlide();
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
  document.addEventListener('wheel', handleScroll);

});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
  document.removeEventListener('wheel', handleScroll);
});
</script>

<template>
  <div class="presentation-fullscreen">

    <div class="slide-container">
      <div class="slide-content" v-html="slides[currentSlideIndex]"></div>
    </div>

      <span class="slide-counter">{{ currentSlideIndex + 1 }} / {{ slides.length }}</span>

    <!-- <div class="keyboard-help">
      <p>Scroller de bas en haut, fleches haut / bas ou gauche/droite pour naviguer | La toucheEsc : Quitter</p>
    </div> -->
  </div>
</template>

<style scoped>
.presentation-fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: #1a1a1a;
  z-index: 9999;
  display: flex;
  flex-direction: column;
}

.slide-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
  overflow: auto;
}

.slide-content {
  width: 100%;
  max-width: 1200px;
  height: 100%;
  background: white;
  padding: 60px;
  border-radius: 8px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  overflow: auto;
  font-size: 18px;
  line-height: 1.6;
}

.slide-content h1,
.slide-content h2,
.slide-content h3 {
  margin-top: 0;
  color: #333;
}

.slide-content p {
  margin: 16px 0;
}

.slide-content ul,
.slide-content ol {
  margin: 16px 0;
  padding-left: 24px;
}

.slide-content li {
  margin: 8px 0;
}

/* .slide-content code {
  background: #f4f4f4;
  padding: 2px 6px;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', monospace;
} */

.slide-counter {
  color: white;
  font-size: 16px;
  font-weight: 500;
  min-width: 100px;
  text-align: center;
}

.keyboard-help {
  position: absolute;
  bottom: 70px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 12px;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.presentation-fullscreen:hover .keyboard-help {
  opacity: 1;
}

@media (max-width: 768px) {
  .slide-content {
    padding: 30px;
    font-size: 16px;
  } 
}
</style>
