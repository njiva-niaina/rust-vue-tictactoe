import { invoke } from "@tauri-apps/api/tauri";
import { defineStore } from "pinia";
import { reactive, ref } from "vue";

export const useGameState = defineStore("game", () => {
  const tab = ref([0, 0, 0, 0, 0, 0, 0, 0, 0]);
  const gameCounter = ref(0);
  const isGameOver = ref(false);
  const winner = ref<Player | null>(null);
  const player = ref(1);

  const players = reactive({
    "1": {
      name: "Player 1",
      schema: "Heart",
      score: 0,
    },
    "-1": {
      name: "Player 1",
      schema: "Brain",
      score: 0,
    },
  });

  async function reset() {
    const result = (await invoke("reset")) as ResetResult;
    tab.value = result.tab;
    gameCounter.value = result.game_counter;
    players[1].score = result.score[1];
    players["-1"].score = result.score["-1"];
    isGameOver.value = false;
    winner.value = null;
  }

  async function makeMove(idx: number) {
    if (tab.value[idx] !== 0) return;
    const result = (await invoke("make_move", { idx: idx })) as MoveResult;
    tab.value[idx] = result.player;
    if (!!result.isGameOver) {
      isGameOver.value = true;
      if ((!!result.isWinner && result.player === 1) || result.player === -1) {
        winner.value = players[result.player];
      }
      return;
    }
    console.log(player.value);
    player.value = -player.value;
    console.log(player.value);
    return;
  }

  return {
    tab,
    gameCounter,
    players,
    isGameOver,
    winner,
    player,
    reset,
    makeMove,
  };
});
