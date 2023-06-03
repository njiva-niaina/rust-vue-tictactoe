<script setup lang="ts">
import Board from "./elements/Board.vue";
import Player from "./elements/Player.vue";
</script>

<template>
  <div class="tic-tac-toe">
    <Board />
    <Player />
  </div>
</template>

<style scoped>
.tic-tac-toe {
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
  height: 100vh;
}
</style>
