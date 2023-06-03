<script setup lang="ts">
import { storeToRefs } from "pinia";

import ModalDialog from "./components/ModalDialog.vue";
import TicTacToe from "./components/TicTacToe.vue";
import GameResult from "./components/elements/GameResult.vue";
import { useGameState } from "./store/gameState";

const gameState = useGameState();
const { isGameOver } = storeToRefs(gameState);
</script>

<template>
  <ModalDialog :display="isGameOver">
    <template #dialog-content>
      <GameResult />
    </template>
  </ModalDialog>
  <TicTacToe />
</template>

<style scoped></style>
