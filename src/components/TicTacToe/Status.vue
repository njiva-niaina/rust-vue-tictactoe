<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useGameStore } from "../../stores/gameStore";

const gameStore = useGameStore();
const { isGameOver, winner } = storeToRefs(gameStore);
</script>

<template>
  <div v-show="isGameOver" class="status">
    <span v-if="winner">
      <span :class="winner.schema === 'X' ? 'yellow' : 'purple'">{{
        winner.schema
      }}</span>
      player win
    </span>
    <span v-else>Match null</span>
  </div>
</template>

<style scoped>
.status {
  position: fixed;
  left: 50%;
  top: 42%;
  transform: translate(-50%, -50%);
  width: 300px;
  height: 300px;
  line-height: 300px;
  text-align: center;
  background-color: rgba(255, 255, 255, 0.5);
  font-size: 24px;
  letter-spacing: 2px;
  pointer-events: none;
}
</style>
