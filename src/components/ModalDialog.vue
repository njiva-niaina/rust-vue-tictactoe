<script setup lang="ts">
import GameResult from "./elements/GameResult.vue";
const props = defineProps<{
  display: boolean;
}>();
</script>

<template>
  <Teleport to="body">
    <div role="dialog" class="modal" :class="display ? 'show' : ''">
      <div class="dialog" :class="display ? 'show' : ''">
        <GameResult />
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.modal {
  z-index: -10;
  position: absolute;
  top: 0;
  width: 100%;
  opacity: 0;
  background-color: rgba(0, 0, 0, 0.2);
  transition: all 1s ease;
}

.modal.show {
  z-index: 10;
  bottom: 0;
  opacity: 1;
}

.dialog {
  opacity: 0;
  position: fixed;
  bottom: 0;
  right: 50%;
  transform: translateX(50%);
  min-width: 24em;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
  border-radius: 0.5em;
  padding: 16px;
  transition: all 1s ease;
}

.dialog.show {
  z-index: 100;
  opacity: 1;
  bottom: 45%;
  background-color: #ffffff;
}
</style>
