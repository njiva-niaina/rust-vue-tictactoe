<script setup lang="ts">
import { useI18n } from "vue-i18n";

import CButton from "@/components/forms/CButton.vue";
import Flag from "@/components/icons/Flag.vue";
import { useModalNavigation } from "@/composable/modalNavigation";
import { useTranslation } from "@/composable/translation";
import { ComputedRef, computed, ref } from "vue";

const { locale } = useI18n();

const modalNavigation = useModalNavigation();
const translation = useTranslation();

const flags  = ref<{
  id: number;
  flag: "fr" | "en" | "mg";
  size: string;
  isSelected: ComputedRef<boolean>;
  clickHandler: Function;
}[]>([
  {
    id: 1,
    flag: "fr",
    size: "40px",
    isSelected: computed(() => locale.value === "fr"),
    clickHandler: () => translation.switchLanguage("fr"),
  },
  {
    id: 2,
    flag: "en",
    size: "40px",
    isSelected: computed(() => locale.value === "en"),
    clickHandler: () => translation.switchLanguage("en"),
  },
  {
    id: 3,
    flag: "mg",
    size: "40px",
    isSelected: computed(() => locale.value === "mg"),
    clickHandler: () => translation.switchLanguage("mg"),
  },
]);
</script>

<template>
  <div class="setting">
    <span>{{ $t("setting.title") }}</span>
    <div class="lang-container">
      <p>{{ $t("setting.lang") }}</p>
      <div class="lang">
        <Flag
          v-for="flag in flags"
          :key="flag.id"
          :flag="flag.flag"
          :size="flag.size"
          :is-selected="flag.isSelected"
          @click="flag.clickHandler"
        />
      </div>
    </div>
    <CButton
      width="18em"
      :content="$t('setting.back')"
      @click="modalNavigation.navigateToComponent('Home')"
    />
  </div>
</template>

<style scoped>
.setting {
  padding: 14px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

span {
  font-size: 32px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 2px;
}

.lang-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-bottom: 2em;
}

.lang {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
}

p {
  font-size: 22px;
}
</style>
