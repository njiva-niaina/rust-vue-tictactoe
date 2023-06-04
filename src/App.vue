<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from "pinia";

import ModalDialog from "./components/ModalDialog.vue";
import TicTacToe from "./components/TicTacToe.vue";
import { useGameState } from "./store/gameState";

const gameState = useGameState();
const { isGameOver } = storeToRefs(gameState);
</script>

<template>
  <TicTacToe />
  <ModalDialog :display="isGameOver" />
</template>

<style scoped>
button {
  z-index: 100;
}
</style>
