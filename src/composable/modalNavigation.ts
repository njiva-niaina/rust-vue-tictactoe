import { storeToRefs } from "pinia";

import { useGameStore } from "@/store/gameStore";
import { useModalStore } from "@/store/modalStore";

export const useModalNavigation = () => {
  const gameStore = useGameStore();
  const modalStore = useModalStore();

  function navigateToComponent(
    component: "Home" | "Setting" | "Result" | "GameLevel"
  ) {
    modalStore.setShowModal({ display: true, component: component });
  }

  function navigateToSetting() {
    navigateToComponent("Setting");
  }

  function playGame(isSinglePlayer: boolean) {
    if (!isSinglePlayer) {
      gameStore.setSinglePlayerMode(false);
      gameStore.resetWithoutCounter();
    } else {
      gameStore.setSinglePlayerMode(true);
      navigateToComponent("GameLevel");
    }
  }

  function selectGameLevel(level: number) {
    gameStore.setLevel(level);
    gameStore.resetWithoutCounter();
  }

  return {
    navigateToSetting,
    navigateToComponent,
    playGame,
    selectGameLevel,
  };
};
