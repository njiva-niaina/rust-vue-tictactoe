<script setup lang="ts">
const props = defineProps<{
  display: boolean;
}>();
</script>

<template>
  <Teleport to="body">
    <div role="dialog" class="modal" :class="display ? 'show' : ''">
      <div class="dialog" :class="display ? 'show' : ''">
        <slot name="dialog-content"></slot>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.modal {
  display: none;
  position: fixed;
  bottom: 0;
  right: 0;
  opacity: 0;
  background-color: transparent;
  transition: all 2s ease;
}

.modal.show {
  display: block;
  top: 0;
  left: 0;
  opacity: 1;
  background-color: rgba(255, 255, 255, 0.35);
}

.dialog {
  min-width: 24em;
  min-height: 12em;
  z-index: 100;
  opacity: 0;
  position: fixed;
  top: 75%;
  left: 50%;
  transform: translate(-50%, -50%);
  background-color: #ffffff;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
  border-radius: 0.5em;
  padding: 8px;
  transition: all 2s ease;
}

.dialog.show {
  opacity: 100;
  top: 40%;
}
</style>
