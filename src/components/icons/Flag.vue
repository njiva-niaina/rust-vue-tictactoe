<script setup lang="ts">
const props = defineProps<{
  flag: "fr" | "en" | "mg";
  size: string;
  isSelected: boolean;
}>();

function getImgUrl() {
  return new URL(
    `../../assets/images/${props.flag}.png`,
    import.meta.url
  ).toString();
}
</script>

<template>
  <img
    :src="getImgUrl()"
    :class="isSelected ? 'selected' : ''"
    alt=""
    srcset=""
  />
</template>

<style scoped>
img {
  padding: 1px;
  cursor: pointer;
  height: v-bind(size);
  border-radius: 4px;
  border: 3px solid rgba(0, 0, 0, 0.2);
}

img.selected {
  border: 3px solid #f4c531;
}
</style>
