<script setup lang="ts">
defineProps<{
  active: boolean;
}>();
const emit = defineEmits(["swicth"]);
</script>
<template>
  <div
    @click="emit('swicth')"
    class="switcher"
    :class="active ? 'active' : ''"
  >
    <div class="circle" :class="active ? 'active' : ''"></div>
  </div>
</template>

<style scoped>
.switcher {
  width: 2rem;
  height: 1rem;
  display: flex;
  align-items: center;
  border-radius: 75px 75px;
  background-color: rgba(0, 0, 0, 0.2);
  transition: all 0.5s ease;
  cursor: pointer;
}

.switcher.active {
  background-color: green;
}

.circle {
  margin: 0.25rem;
  height: 0.75rem;
  width: 0.75rem;
  background-color: #ffffff;
  border-radius: 50%;
  transition: all 0.5s ease;
}

.circle.active {
  transform: translateX(100%);
}
</style>
