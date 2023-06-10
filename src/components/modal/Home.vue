<script setup lang="ts">
import { exit } from "@tauri-apps/api/process";

import CButton from "@/components/forms/CButton.vue";
import { useModalNavigation } from "@/composable/modalNavigation.js";

const modalNavigation = useModalNavigation();

const buttons = [
  {
    id: 1,
    content: "home.singlePlayer",
    clickHandler: () => modalNavigation.playGame(true),
  },
  {
    id: 2,
    content: "home.multiplayer",
    clickHandler: () => modalNavigation.playGame(false),
  },
  {
    id: 3,
    content: "home.setting",
    clickHandler: modalNavigation.navigateToSetting,
  },
  {
    id: 4,
    content: "home.exit",
    clickHandler: async () => await exit(),
  },
];
</script>

<template>
  <div class="home">
    <span>{{ $t("home.title") }}</span>
    <CButton
      v-for="btn in buttons"
      :key="btn.id"
      :content="btn.content"
      @click="btn.clickHandler"
      width="18em"
    />
  </div>
</template>

<style scoped>
.home {
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
</style>
