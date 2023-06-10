<script setup lang="ts">
import CButton from "@/components/forms/CButton.vue";
import { useModalNavigation } from "@/composable/modalNavigation";


const modalNavigation = useModalNavigation();

const buttons = [
  {
    id: 1,
    content: "level.easy",
    clickHandler: () => modalNavigation.selectGameLevel(0),
  },
  {
    id: 2,
    content: "level.intermediate",
    clickHandler: () => modalNavigation.selectGameLevel(1),
  },
  {
    id: 3,
    content: "level.impossible",
    clickHandler: () => modalNavigation.selectGameLevel(6),
  },
];
</script>

<template>
  <div class="game-level">
    <span>{{ $t("level.title") }}</span>
    <CButton
      v-for="btn in buttons"
      width="18em"
      :key="btn.id"
      :content="btn.content"
      @click="btn.clickHandler"
    />
    <div class="separator"></div>
    <CButton
      width="18em"
      content="level.back"
      @click="modalNavigation.navigateToComponent('Home')"
    />
  </div>
</template>

<style scoped>
.game-level {
  padding: 14px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1em;
}

span {
  font-size: 32px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 2px;
}

.separator {
  margin: 3px;
}
</style>
