<script setup lang="ts">
const props = defineProps<{
  content: string;
  width: string
}>();
const emit = defineEmits(["click"]);
</script>

<template>
  <button @click="emit('click')">
    {{ content }}
  </button>
</template>
<style scoped>
button {
  position: relative;
  width: v-bind(width);
  color: #ffffff;
  font-size: 18px;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-weight: 600;
  border-radius: 5px;
  border: solid 1px #f1c32b;
  background: #f4c531;
  text-align: center;
  padding: 16px 18px 14px;
  transition: all 0.015s;
  box-shadow: 0px 6px 0px #cfa004;
  cursor: pointer;
}

button:active {
  box-shadow: 0px 2px 0px #cfa004;
  position: relative;
  top: 4px;
}
</style>
