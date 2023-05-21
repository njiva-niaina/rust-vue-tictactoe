<script setup lang="ts">
import Grid from "../components/TicTacToe/Grid.vue";
import Status from "../components/TicTacToe/Status.vue";
import TitleBar from "../components/TicTacToe/TitleBar.vue";
import Score from "../components/TicTacToe/Score.vue";
</script>

<template>
  <div class="tic-tac-toe">
    <TitleBar />
    <Status />
    <Grid />
    <Score />
  </div>
</template>

<style scoped>
.tic-tac-toe {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  height: 100vh;
}
</style>
