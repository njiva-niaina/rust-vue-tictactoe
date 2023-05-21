type TauriDrawResult = {
  game: { IsOver: boolean };
  win: { Status: boolean };
  player: { PlayerIdx: number };
};

type TauriResetResult = {
  status: { GameIsOver: boolean };
  game: { Counter: number };
  node: { Tab: number[] };
  players: { Score: number[] };
};

type Player = {
  schema: string;
  name: string;
  score: number;
};
