<script setup lang="ts">
const props = defineProps<{
  width: number;
  height: number;
}>();
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="120"
    height="12"
  >
    <rect width="12" height="12" fill="#CE1126" />
    <rect width="12" height="12" fill="#FFFFFF" />
    <rect width="120" height="12" fill="#002654" />
  </svg>
</template>
