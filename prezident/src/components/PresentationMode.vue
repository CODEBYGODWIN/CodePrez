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
  }
};

const handleScroll = (event: WheelEvent) => {
  if (event.deltaY > 0) {
    nextSlide();
  } else if (event.deltaY < 0) {
    previousSlide();
  }
};

const handleFullscreenChange = () => {
  if (!document.fullscreenElement) {
    emit('close');
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
  document.addEventListener('wheel', handleScroll);
  document.addEventListener('fullscreenchange', handleFullscreenChange);

  document.documentElement.requestFullscreen().catch(err => {
    console.warn('Fullscreen non disponible :', err);
  });
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
  document.removeEventListener('wheel', handleScroll);
  document.removeEventListener('fullscreenchange', handleFullscreenChange);

  if (document.fullscreenElement) {
    document.exitFullscreen();
  }
});
</script>

<template>
  <div class="presentation-fullscreen">
    <div class="slide-container">
      <div class="slide-content" v-html="slides[currentSlideIndex]"></div>
    </div>
    <span class="slide-counter">{{ currentSlideIndex + 1 }} / {{ slides.length }}</span>
  </div>
</template>

<style scoped>
* {
  margin: 0;
  padding: 0;
}

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

.slide-counter {
  color: white;
  font-size: 16px;
  font-weight: 500;
  min-width: 100px;
  text-align: center;
  padding-bottom: 16px;
}
</style>