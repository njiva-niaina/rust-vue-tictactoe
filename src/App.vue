<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from "pinia";

import ModalDialog from "./components/ModalDialog.vue";
import TicTacToe from "./components/TicTacToe.vue";
import PlusIcon from "./components/icons/PlusIcon.vue";
import { useGameState } from "./store/gameState";

const showSettings = ref(false);
const gameState = useGameState();
const { isGameOver } = storeToRefs(gameState);
</script>

<template>
  <PlusIcon :active="showSettings" @display="showSettings = !showSettings" />
  <TicTacToe :show-settings="showSettings" />
  <ModalDialog :display="isGameOver" />
</template>

<style scoped>
button {
  z-index: 100;
}
</style>
