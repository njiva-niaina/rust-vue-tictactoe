<script setup lang="ts">
import { storeToRefs } from "pinia";

import BoardItem from "@/components/elements/BoardItem.vue";
import { useGameStore } from "@/store/gameStore";

const gameStore = useGameStore();
const { tab } = storeToRefs(gameStore);
</script>

<template>
  <div class="board">
    <BoardItem
      v-for="(value, index) in tab"
      :id="index"
      :idx="index"
      :value="value"
    />
  </div>
</template>

<style scoped>
.board {
  display: inline-grid;
  grid-template-columns: repeat(3, auto);
  grid-template-rows: repeat(3, auto);
  grid-column-gap: 0.5em;
  grid-row-gap: 0.5em;
  justify-content: center;
  padding: 1em;
  border-radius: 0.6em;
  background-color: #ffffff;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
}
</style>
