<script lang="ts" setup>
import { storeToRefs } from "pinia";

import { useGameStore } from "../../stores/gameStore";

const gameStore = useGameStore();
const { isGameOver } = storeToRefs(gameStore);
</script>

<template>
  <div class="title-bar">
    <button
      class="reset-button"
      :disabled="!isGameOver"
      @click="gameStore.reset"
    >
      Reset
    </button>
    <span class="title">Tic Tac Toe</span>
  </div>
</template>

<style scoped>
.title-bar {
  width: 100%;
  margin: 12px;
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
  align-items: center;
}
.reset-button {
  padding: 12px;
  color: #123463;
  font-size: 16px;
  font-weight: 600;
  border: none;
  background: none;
  border-radius: 8px;
  cursor: pointer;
}
.reset-button:disabled,
.reset-button[disabled] {
  background-color: rgba(0, 0, 0, 0.1);
  color: #666666;
  pointer-events: none;
}
.reset-button:hover {
  background-color: rgba(0, 0, 0, 0.1);
}
.title {
  font-size: 24px;
  font-weight: bold;
  letter-spacing: 2px;
}
</style>
