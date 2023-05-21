<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useGameStore } from "../../stores/gameStore";
import GridItem from "./GridItem.vue";

const gameStore = useGameStore();
const { tab } = storeToRefs(gameStore);
</script>

<template>
  <div class="grid">
    <GridItem
      v-for="(value, index) in tab"
      :id="index"
      :idx="index"
      :value="value"
    />
  </div>
</template>

<style scoped>
.grid {
  display: inline-grid;
  grid-template-rows: 100px 100px 100px;
  grid-template-columns: 100px 100px 100px;
}
</style>
