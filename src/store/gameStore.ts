import { invoke } from "@tauri-apps/api/tauri";
import { defineStore } from "pinia";
import { reactive, ref } from "vue";

import { useModalStore } from "./modalStore";

export const useGameStore = defineStore("game", () => {
  const modalStore = useModalStore();

  const tab = ref([0, 0, 0, 0, 0, 0, 0, 0, 0]);
  const gameCounter = ref(0);
  const isGameOver = ref(false);
  const winner = ref<Player | null>(null);
  const player = ref(1);
  const isSinglePlayer = ref(false);
  const level = ref(0);

  const players = reactive({
    "1": {
      name: "game.player1",
      schema: "Heart",
      score: 0,
    },
    "-1": {
      name: "game.player2",
      schema: "Brain",
      score: 0,
    },
  });

  function setGameStatus(result: MoveResult) {
    if (!!result.isGameOver) {
      modalStore.setShowModal({ display: true, component: "Result" });
      isGameOver.value = true;
      if ((!!result.isWinner && result.player === 1) || result.player === -1) {
        winner.value = players[result.player];
      }
      return;
    }
  }

  function setSinglePlayerMode(_isSinglePlayer: boolean) {
    isSinglePlayer.value = _isSinglePlayer;
  }

  function setLevel(_level: number) {
    level.value = _level;
  }

  async function _reset(fn: Function) {
    const result = (await fn()) as ResetResult;

    tab.value = result.tab;
    gameCounter.value = result.game_counter;
    players[1].score = result.score[1];
    players["-1"].score = result.score["-1"];
    player.value = result.player;

    isGameOver.value = false;
    winner.value = null;
    modalStore.setShowModal({ display: false });
  }

  async function resetWithoutCounter() {
    _reset(() => invoke("reset_without_counter"));
  }

  async function reset() {
    _reset(() => invoke("reset"));
  }

  async function makeMove(idx: number) {
    if (tab.value[idx] !== 0) return;

    const result = (await invoke("make_move", { idx: idx })) as MoveResult;
    tab.value[idx] = result.player;

    setGameStatus(result);

    player.value = -player.value;

    if (isSinglePlayer.value) {
      makeBestMove();
    }

    return;
  }

  async function makeBestMove() {
    const bestMoveIdx = await findBestMove();
    if (player.value === -1 && bestMoveIdx !== -1) {
      setTimeout(() => {
        makeMove(bestMoveIdx);
      }, 550);
    }
  }

  async function findBestMove(): Promise<number> {
    if (player.value !== -1 || isGameOver.value) return -1;
    const bestMove = (await invoke("find_best_move", {
      player: player.value,
      depth: level.value,
    })) as number;
    return bestMove;
  }

  return {
    tab,
    gameCounter,
    players,
    isGameOver,
    winner,
    player,
    isSinglePlayer,
    setLevel,
    reset,
    makeMove,
    setSinglePlayerMode,
    resetWithoutCounter,
  };
});
