<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useModalStore } from "@/store/modalStore";

const modalStore = useModalStore();
const { showModal, currentModalContent } = storeToRefs(modalStore);
</script>

<template>
  <Teleport to="body">
    <div role="dialog" class="modal" :class="showModal ? 'show' : ''">
      <div class="dialog" :class="showModal ? 'show' : ''">
        <component v-if="currentModalContent" :is="currentModalContent" />
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
  min-height: 18em;
  box-shadow: 0 0.9em 2.8em rgba(86, 66, 0, 0.2);
  border-radius: 0.5em;
  padding: 16px;
  transition: all 1s ease;
}

.dialog.show {
  z-index: 100;
  opacity: 1;
  bottom: 35%;
  background-color: #ffffff;
}
</style>
