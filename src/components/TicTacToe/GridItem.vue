<script setup lang="ts">
import { computed } from "vue";

import { useGameStore } from "../../stores/gameStore";

const props = defineProps<{
  idx: number;
  value: number;
}>();

const gameStore = useGameStore();

async function draw() {
  // Box is not empty
  if (!!props.value) return;
  gameStore.draw(props.idx);
}

const borderLeftStyle = computed(() =>
  (props.idx + 3) % 3 === 0 ? "no-border-left" : ""
);
const borderTopStyle = computed(() => (props.idx <= 2 ? "no-border-top" : ""));
const borderRightStyle = computed(() =>
  (props.idx + 1) % 3 === 0 ? "no-border-right" : ""
);
const borderBottomStyle = computed(() =>
  props.idx > 5 ? "no-border-bottom" : ""
);
</script>

<template>
  <div
    class="grid-item"
    :class="`${borderLeftStyle} ${borderTopStyle} ${borderRightStyle} ${borderBottomStyle}`"
    @click="draw"
  >
    <span v-if="value === 0"></span>
    <span v-if="value === 1" class="yellow">X</span>
    <span v-if="value === 2" class="purple">O</span>
  </div>
</template>

<style scoped>
.grid-item {
  border: 1px solid black;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  font-size: 64px;
  font-weight: bold;
}

.no-border-left {
  border-left: none;
}
.no-border-top {
  border-top: none;
}
.no-border-right {
  border-right: none;
}
.no-border-bottom {
  border-bottom: none;
}
</style>
