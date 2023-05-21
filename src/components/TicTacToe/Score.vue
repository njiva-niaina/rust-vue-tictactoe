<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useGameStore } from "../../stores/gameStore";
import Game from "./Game.vue";
import Player from "./Player.vue";

const gameStore = useGameStore();
const { players } = storeToRefs(gameStore);
</script>

<template>
  <div class="score">
    <Player
      :schema="players[0].schema"
      :name="players[0].name"
      :score="players[0].score"
    />
    <Game />
    <Player
      :schema="players[1].schema"
      :name="players[1].name"
      :score="players[1].score"
    />
  </div>
</template>

<style scoped>
.score {
  width: 100%;
  height: 25%;
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
  align-items: center;
  background-color: rgba(0, 0, 0, 0.2);
}
</style>
