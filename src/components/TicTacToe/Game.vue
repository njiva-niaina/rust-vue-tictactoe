<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useGameStore } from "../../stores/gameStore";

const gameStore = useGameStore();
const { counter } = storeToRefs(gameStore);
</script>

<template>
  <div class="game">
    <span class="text-sm">Games</span>
    <span>{{ counter }}</span>
  </div>
</template>

<style scoped>
.game {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
  font-size: 64px;
  font-weight: bold;
}
</style>
