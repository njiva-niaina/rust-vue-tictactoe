<script lang="ts" setup>
import { storeToRefs } from "pinia";

import { useGameStore } from "@/store/gameStore";
import Brain from "@/components/icons/Brain.vue";
import Heart from "@/components/icons/Heart.vue";

const gameStore = useGameStore();
const { winner } = storeToRefs(gameStore);
</script>

<template>
  <div class="game-result">
    <div class="win" v-if="winner">
      <Heart v-if="winner.schema === 'Heart'" :height="128" :width="128" />
      <Brain v-else :height="128" :width="128" />
      <div class="legend">Win</div>
    </div>
    <div class="null" v-else>
      <div class="inline">
        <Heart :height="128" :width="128" />
        <Brain :height="128" :width="128" />
      </div>
      {{ $t("home.singlePlayer") }}
      <div class="legend">Match null</div>
    </div>
    <button @click="gameStore.reset">Rejouer</button>
  </div>
</template>

<style scoped>
.game-result,
.win,
.null {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.game-result {
  padding: 16px;
}

.inline {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

.legend {
  text-transform: uppercase;
  font-size: 32px;
  font-weight: bolder;
}

button {
  margin-top: 16px;
  border: none;
  background: none;
  color: #000;
  cursor: pointer;
}
</style>
