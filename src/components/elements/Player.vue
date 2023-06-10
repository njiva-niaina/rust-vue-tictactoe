<script setup lang="ts">
import { storeToRefs } from "pinia";
import { computed, ref } from "vue";

import Brain from "@/components/icons/Brain.vue";
import Heart from "@/components/icons/Heart.vue";
import { useGameStore } from "@/store/gameStore";

const gameStore = useGameStore();
const { player, players, gameCounter } = storeToRefs(gameStore);

const infos = ref([
  {
    id: 1,
    title: computed(() => players.value[1].name),
    value: computed(() => players.value[1].score),
  },
  {
    id: 2,
    title: "game.counter",
    value: computed(() => gameCounter.value),
  },
  {
    id: 3,
    title: computed(() => players.value["-1"].name),
    value: computed(() => players.value["-1"].score),
  },
]);
</script>

<template>
  <div class="player-container">
    <div class="line" :class="player === -1 ? 'second' : ''"></div>
    <div class="player">
      <div v-for="info in infos" :key="info.id" class="player-info">
        <Heart v-if="info.id === 1" :height="50" :width="50" />
        <Brain v-if="info.id === 3" :height="50" :width="50" />
        <span class="player-name">{{ $t(info.title) }}</span>
        <span class="player-score">{{ info.value }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-container {
  background-color: #ffffff;
  width: 40%;
  display: flex;
  flex-direction: column;
  border-radius: 0 0 0.6em 0.6em;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
}

.line {
  background-color: #f24d11;
  height: 6px;
  width: 35%;
  transition: all 1s ease;
}

.line.second {
  background-color: #ff8195;
  transform: translateX(186.5%);
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
