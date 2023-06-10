<script setup lang="ts">
import { ref, watch } from "vue";

import Brain from "@/components/icons/Brain.vue";
import Heart from "@/components/icons/Heart.vue";
import { useGameStore } from "@/store/gameStore";

const props = defineProps<{
  idx: number;
  value: number;
}>();

const gameStore = useGameStore();

const iconSize = ref({
  heart: {
    height: 0,
    width: 0,
  },
  brain: {
    height: 0,
    width: 0,
  },
});

async function makeMove() {
  // Box is not empty
  if (!!props.value) return;
  await gameStore.makeMove(props.idx);
}

function updateIconSize({
  iconKey,
  size,
}: {
  iconKey: "brain" | "heart";
  size: { height: number; width: number };
}) {
  iconSize.value[iconKey] = size;
}

function resetIconSize() {
  updateIconSize({ iconKey: "brain", size: { height: 0, width: 0 } });
  updateIconSize({ iconKey: "heart", size: { height: 0, width: 0 } });
}

watch(props, () => {
  if (props.value === 1) {
    updateIconSize({ iconKey: "heart", size: { width: 50, height: 50 } });
  }
  if (props.value === -1) {
    updateIconSize({ iconKey: "brain", size: { width: 50, height: 50 } });
  }
  if (props.value === 0) {
    resetIconSize();
  }
});
</script>

<template>
  <div
    @click="makeMove"
    class="board-item"
    :class="value === 0 ? 'bg-yellow' : 'bg-white'"
  >
    <Brain :height="iconSize.brain.height" :width="iconSize.brain.height" />
    <Heart :height="iconSize.heart.height" :width="iconSize.heart.width" />
  </div>
</template>

<style scoped>
.board-item {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 5em;
  height: 5em;
  cursor: pointer;
  font-weight: bold;
  border-radius: 5px;
  border: 4px solid #000000;
  transition: background-color 0.2s linear;
}

.board-item:hover {
  background-color: #ffffff;
}

.bg-white {
  background-color: #ffffff;
}

.bg-yellow {
  background-color: #f4c531;
}
</style>
