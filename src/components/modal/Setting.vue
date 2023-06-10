<script setup lang="ts">
import { useI18n } from "vue-i18n";

import CButton from "@/components/forms/CButton.vue";
import ENFlag from "@/components/icons/ENFlag.vue";
import { useModalNavigation } from "@/composable/modalNavigation";
import { useTranslation } from "@/composable/translation";

const { locale } = useI18n();

const modalNavigation = useModalNavigation();
const translation = useTranslation();
</script>

<template>
  <div class="setting">
    <span>{{ $t("setting.title") }}</span>
    <div class="lang-container">
      <p>{{ $t("setting.lang") }}</p>
      <div class="lang">
        <ENFlag
          :is-selected="locale === 'en'"
          @click="translation.switchLanguage('en')"
        />
        <ENFlag
          :is-selected="locale === 'fr'"
          @click="translation.switchLanguage('fr')"
        />
        <ENFlag
          :is-selected="locale === 'mg'"
          @click="translation.switchLanguage('mg')"
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
