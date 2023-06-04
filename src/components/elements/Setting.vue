<script lang="ts" setup>
import { ref } from "vue";

defineProps<{
  show: boolean
}>();

const active = ref(false);
</script>

<template>
  <div class="setting" :class="show ? 'show' : ''">
    <div class="setting-content"></div>
  </div>
</template>

<style scoped>
.setting {
  height: 100%;
  width: 0;
  opacity: 0;
  background-color: #ffffff;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
  transition: all 1s ease;
}

.setting.show {
  opacity: 1;
  width: 35%;
}
</style>
