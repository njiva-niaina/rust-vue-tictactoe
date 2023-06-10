<script setup lang="ts">
import CButton from "@/components/forms/CButton.vue";
import MGFlag from "@/components/icons/MGFlag.vue";
import ENFlag from "@/components/icons/ENFlag.vue";
import FRFlag from "@/components/icons/FRFlag.vue";

import { useModalNavigation } from "@/composable/modalNavigation";

const modalNavigation = useModalNavigation();
</script>

<template>
  <div class="setting">
    <span>{{ $t("setting.title") }}</span>
    <div class="lang-container">
      <p>{{ $t("setting.lang") }}</p>
      <div class="lang">
        <MGFlag :width="154" :height="130" /> --
        <ENFlag :width="54" :height="30" /> --
        <FRFlag :width="54" :height="30" /> --
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
  gap: 1em;
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
}

.lang {
    display: flex;
    flex-direction: row;
}
</style>
