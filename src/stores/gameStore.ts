import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export const useGameStore = defineStore("game", () => {
  const tab = ref([0, 0, 0, 0, 0, 0, 0, 0, 0]);
  const counter = ref(0);
  const isGameOver = ref(false);
  const winner = ref<Player | null>(null);

  const players = reactive([
    {
      schema: "X",
      name: "Player 1",
      score: 0,
    },
    {
      schema: "O",
      name: "Player 1",
      score: 0,
    },
  ]);

  const setGameOverStatus = (status: boolean) => {
    isGameOver.value = status;
  };

  const setWinner = (winnerIdx: number | null) => {
    setGameOverStatus(true);
    if (winnerIdx === null) {
      winner.value = null;
      return;
    }
    winner.value = players[winnerIdx];
  };

  const setPlayerScore = ({
    playerIdx,
    score,
  }: {
    playerIdx: number;
    score: number;
  }) => (players[playerIdx].score = score);

  const setTab = ({ idx, value }: { idx: number; value: number }) => {
    tab.value[idx] = value;
  };

  async function draw(idx: number) {
    const result = (await invoke("draw", { idx: idx })) as TauriDrawResult;
    if (!result.player.PlayerIdx) return;
    setTab({ idx: idx, value: result.player.PlayerIdx });
    if (result.win.Status) {
      setWinner(result.player.PlayerIdx - 1);
      return;
    }
    console.log(result.game.IsOver && !result.win.Status);
    if (result.game.IsOver && !result.win.Status) {
      setWinner(null);
      return;
    }
    return;
  }

  async function reset() {
    if (!isGameOver.value) return;

    const result = (await invoke("reset")) as TauriResetResult;

    setPlayerScore({ playerIdx: 0, score: result.players.Score[0] });
    setPlayerScore({ playerIdx: 1, score: result.players.Score[1] });

    tab.value = result.node.Tab;
    counter.value = result.game.Counter;
    isGameOver.value = result.status.GameIsOver;

    return;
  }

  return {
    tab,
    winner,
    counter,
    players,
    isGameOver,
    setWinner,
    setGameOverStatus,
    setTab,
    draw,
    reset,
  };
});
