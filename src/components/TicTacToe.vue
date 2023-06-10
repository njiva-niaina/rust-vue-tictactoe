<script setup lang="ts">
import Board from "@/components/elements/Board.vue";
import Player from "@/components/elements/Player.vue";
</script>

<template>
  <div class="tic-tac-toe">
    <div class="game">
      <Board />
      <Player />
    </div>
  </div>
</template>

<style scoped>
.tic-tac-toe {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  height: 100vh;
  width: 100%;
}

.game {
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
  height: 100%;
  width: 100%;
}
</style>
