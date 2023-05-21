<script setup lang="ts">
const props = defineProps<{
  name: string;
  score: number;
  schema: string;
}>();
</script>

<template>
  <div class="player">
    <div class="player-info text-sm">
      <span :class="$props.schema === 'X' ? 'yellow' : 'purple'">{{
        schema
      }}</span>
      <span>{{ name }}</span>
    </div>
    <span>{{ score }}</span>
  </div>
</template>

<style scoped>
.player {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
  font-size: 64px;
  font-weight: bold;
}

.player-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>
