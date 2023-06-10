import { defineStore } from "pinia";
import { computed, ref } from "vue";

import GameLevel from "@/components/modal/GameLevel.vue";
import Result from "@/components/modal/GameResult.vue";
import Home from "@/components/modal/Home.vue";
import Setting from "@/components/modal/Setting.vue";

export const useModalStore = defineStore("modal", () => {
  const showModal = ref<boolean>(true);
  const currentModalContent = ref<
    "Home" | "Setting" | "Result" | "GameLevel" | undefined
  >("Home");

  function setCurrentModalContent(
    component: "Home" | "Setting" | "Result" | "GameLevel" | undefined
  ) {
    currentModalContent.value = component;
  }

  function setShowModal({
    display,
    component,
  }: {
    display: boolean;
    component?: "Home" | "Setting" | "Result" | "GameLevel";
  }) {
    if (display) {
      setCurrentModalContent(component);
      showModal.value = true;
    } else {
      showModal.value = false;
      setCurrentModalContent(undefined);
    }
  }

  return {
    showModal,
    currentModalContent: computed(() => {
      if (currentModalContent.value === undefined) return;
      if (currentModalContent.value === "Home") return Home
      if (currentModalContent.value === "Setting") return Setting
      if (currentModalContent.value === "Result") return Result
      if (currentModalContent.value === "GameLevel") return GameLevel
    }),
    setShowModal,
  };
});
