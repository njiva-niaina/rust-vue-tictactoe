<script setup lang="ts">
defineProps<{
  active: boolean;
}>();
const emit = defineEmits(["display"]);
</script>

<template>
  <div @click="emit('display')" class="icon" :class="active ? 'active' : ''">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      color="white"
      width="24px"
      height="24px"
      fill-rule="evenodd"
    >
      <path
        fill-rule="evenodd"
        fill="currentColor"
        d="M 11 2 L 11 11 L 2 11 L 2 13 L 11 13 L 11 22 L 13 22 L 13 13 L 22 13 L 22 11 L 13 11 L 13 2 Z"
      />
    </svg>
  </div>
</template>

<style scoped>
.icon svg {
  cursor: pointer;
  position: fixed;
  top: 25px;
  right: 25px;
  color: #ffffff;
  transition: transform 1s ease;
}

.icon.active svg {
  transform: rotate(45deg);
  color: #000;
}
</style>
