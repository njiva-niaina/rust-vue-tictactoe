<script setup lang="ts">
import { useI18n } from "vue-i18n";

import CButton from "@/components/forms/CButton.vue";
import { useModalNavigation } from "@/composable/modalNavigation";

const { t } = useI18n();

const modalNavigation = useModalNavigation();

const buttons = [
  {
    id: 1,
    content: t("level.easy"),
    clickHandler: () => modalNavigation.selectGameLevel(3),
  },
  {
    id: 2,
    content: t("level.intermediate"),
    clickHandler: () => modalNavigation.selectGameLevel(5),
  },
  {
    id: 3,
    content: t("level.impossible"),
    clickHandler: () => modalNavigation.selectGameLevel(9),
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
      :content="$t('level.back')"
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
