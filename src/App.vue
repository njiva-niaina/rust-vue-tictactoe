<script setup lang="ts">
import ModalDialog from "@/components/ModalDialog.vue";
import TicTacToe from "@/components/TicTacToe.vue";
import GameResult from "@/components/elements/GameResult.vue";
</script>

<template>
  <TicTacToe />
  <ModalDialog :display="true">
    <component :is="GameResult" />
  </ModalDialog>
</template>

<style scoped></style>
