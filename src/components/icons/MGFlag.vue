<script setup lang="ts">
const props = defineProps<{
  width: number;
  height: number;
}>();
</script>

<template>
  <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12">
    <rect fill="#007E3A" width="12" height="12" />
    <rect fill="#FC3D32" width="12" height="12" />
    <rect fill="#FFF" width="12" height="12" />
  </svg>
</template>
