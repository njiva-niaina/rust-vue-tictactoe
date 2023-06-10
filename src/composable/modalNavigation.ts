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
      modalStore.setShowModal({ display: false });
    } else {
      gameStore.setSinglePlayerMode(true);
      navigateToComponent("GameLevel");
    }
  }

  return {
    navigateToSetting,
    navigateToComponent,
    playGame,
  };
};
