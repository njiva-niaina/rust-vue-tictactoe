<script lang="ts" setup>
import { storeToRefs } from "pinia";
import { useI18n } from "vue-i18n";

import Brain from "@/components/icons/Brain.vue";
import Heart from "@/components/icons/Heart.vue";
import CButtonVue from "../forms/CButton.vue";
import { useGameStore } from "@/store/gameStore";
import { useModalNavigation } from "@/composable/modalNavigation";

const { t } = useI18n();

const modalNavigation = useModalNavigation();
const gameStore = useGameStore();
const { winner } = storeToRefs(gameStore);

const buttons = [
  {
    id: 1,
    content: t("result.again"),
    clickHandler: () => gameStore.reset(),
  },
  {
    id: 2,
    content: t("result.back"),
    clickHandler: () => modalNavigation.navigateToComponent("Home"),
  },
];
</script>

<template>
  <div class="game-result">
    <div class="win" v-if="winner">
      <Heart v-if="winner.schema === 'Heart'" :height="128" :width="128" />
      <Brain v-else :height="128" :width="128" />
      <div class="legend">Win</div>
    </div>
    <div class="null" v-else>
      <div class="inline">
        <Heart :height="128" :width="128" />
        <Brain :height="128" :width="128" />
      </div>
      <div class="legend">Match null</div>
    </div>
    <CButtonVue
      v-for="btn in buttons"
      width="18em"
      :key="btn.id"
      :content="btn.content"
      @click="btn.clickHandler"
    />
  </div>
</template>

<style scoped>
.game-result,
.win,
.null {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.game-result {
  padding: 16px;
}

.inline {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

.legend {
  text-transform: uppercase;
  font-size: 32px;
  font-weight: bolder;
}

button {
  margin-top: 16px;
}
</style>
