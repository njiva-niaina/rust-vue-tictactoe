<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useGameStore } from "@/store/gameStore";
import Brain from "@/components/icons/Brain.vue";
import Heart from "@/components/icons/Heart.vue";

const gameStore = useGameStore();
const { player, players } = storeToRefs(gameStore);
</script>

<template>
  <div class="player-container">
    <div class="line" :class="player === -1 ? 'second' : ''"></div>
    <div class="player">
      <div v-for="(item, idx) in players" class="player-info">
        <Heart v-if="idx === '1'" :height="50" :width="50" />
        <Brain v-else :height="50" :width="50" />
        <span class="player-name">{{ item.name }}</span>
        <span class="player-score">{{ item.score }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-container {
  background-color: #ffffff;
  width: 60%;
  display: flex;
  flex-direction: column;
  border-radius: 0 0 0.6em 0.6em;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
}

.line {
  background-color: #f24d11;
  height: 6px;
  width: 50%;
  transition: all 1s ease;
}

.line.second {
  background-color: #ff8195;
  transform: translateX(100%);
}

.player {
  display: flex;
  width: 100%;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
  margin-top: 1em;
  margin-bottom: 1em;
}

.player-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: #000;
}

.player-name {
  font-size: large;
}

.player-score {
  font-size: 32px;
  font-weight: bold;
}
</style>
