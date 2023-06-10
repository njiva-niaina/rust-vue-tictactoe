import { storeToRefs } from "pinia";

import { useGameStore } from "@/store/gameStore";
import { useModalStore } from "@/store/modalStore";

export const useModalNavigation = () => {
  const gameStore = useGameStore();
  const modalStore = useModalStore();

  const { isGameOver } = storeToRefs(gameStore);

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
      modalStore.setShowModal({ display: false });
    } else {
      gameStore.setSinglePlayerMode(true);
      navigateToComponent("GameLevel");
    }
  }

  function selectGameLevel(level: number) {
    gameStore.setLevel(level);
    modalStore.setShowModal({ display: false });
  }

  return {
    navigateToSetting,
    navigateToComponent,
    playGame,
    selectGameLevel,
  };
};
