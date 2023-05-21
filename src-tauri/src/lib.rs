use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub enum DrawResult {
    Status(bool),
    IsOver(bool),
    PlayerIdx(usize),
}

#[derive(Debug, serde::Serialize)]
pub enum ResetResult {
    Tab([usize; 9]),
    Counter(i32),
    Score([i32; 2]),
    GameIsOver(bool),
}

#[derive(Debug)]
pub struct Node {
    tab: [usize; 9],
    is_my_turn: bool,
    game: i32,
    score: [i32; 2],
    is_game_over: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            tab: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            is_my_turn: true,
            game: 0,
            score: [0, 0],
            is_game_over: false,
        }
    }

    pub fn reset(&mut self) -> HashMap<String, ResetResult> {
        let mut result: HashMap<String, ResetResult> = HashMap::new();

        self.tab = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        self.game += 1;
        self.is_my_turn = true;
        self.is_game_over = false;

        result.insert(
            String::from("status"),
            ResetResult::GameIsOver(self.is_game_over),
        );
        result.insert(String::from("game"), ResetResult::Counter(self.game));
        result.insert(String::from("node"), ResetResult::Tab(self.tab));
        result.insert(String::from("players"), ResetResult::Score(self.score));

        return result;
    }

    pub fn draw(&mut self, idx: usize) -> HashMap<String, DrawResult> {
        let mut result: HashMap<String, DrawResult> = HashMap::new();

        if idx > self.tab.len() || self.tab[idx] != 0 || self.is_game_over {
            result.insert(String::from("win"), DrawResult::Status(false));
            result.insert(String::from("game"), DrawResult::IsOver(false));
            result.insert(String::from("player"), DrawResult::PlayerIdx(0));
            return result;
        }

        let value: usize = if self.is_my_turn { 1 } else { 2 };
        self.tab[idx] = value;

        if self.does_player_win() {
            result.insert(String::from("win"), DrawResult::Status(true));
            result.insert(String::from("game"), DrawResult::IsOver(true));
            result.insert(String::from("player"), DrawResult::PlayerIdx(value));

            self.is_game_over = true;
            self.score[value - 1] += 1;

            return result;
        }
        if self.is_tab_full() {
            result.insert(String::from("win"), DrawResult::Status(false));
            result.insert(String::from("game"), DrawResult::IsOver(true));
            result.insert(String::from("player"), DrawResult::PlayerIdx(value));

            self.is_game_over = true;

            return result;
        }

        result.insert(String::from("win"), DrawResult::Status(false));
        result.insert(String::from("game"), DrawResult::IsOver(false));
        result.insert(String::from("player"), DrawResult::PlayerIdx(value));

        self.reverse_turn();

        return result;
    }

    fn reverse_turn(&mut self) {
        self.is_my_turn = !self.is_my_turn
    }

    fn is_tab_full(&self) -> bool {
        for item in self.tab.iter() {
            if *item == 0 {
                return false;
            }
        }
        return true;
    }

    fn does_player_win(&self) -> bool {
        if self.tab[0] != 0 && self.tab[0] == self.tab[1] && self.tab[1] == self.tab[2] {
            return true;
        }
        if self.tab[3] != 0 && self.tab[3] == self.tab[4] && self.tab[4] == self.tab[5] {
            return true;
        }
        if self.tab[6] != 0 && self.tab[6] == self.tab[7] && self.tab[7] == self.tab[8] {
            return true;
        }

        if self.tab[0] != 0 && self.tab[0] == self.tab[3] && self.tab[3] == self.tab[6] {
            return true;
        }
        if self.tab[1] != 0 && self.tab[1] == self.tab[4] && self.tab[4] == self.tab[7] {
            return true;
        }
        if self.tab[2] != 0 && self.tab[2] == self.tab[5] && self.tab[5] == self.tab[8] {
            return true;
        }

        if self.tab[0] != 0 && self.tab[0] == self.tab[4] && self.tab[4] == self.tab[8] {
            return true;
        }
        if self.tab[2] != 0 && self.tab[2] == self.tab[4] && self.tab[4] == self.tab[6] {
            return true;
        }

        return false;
    }
}
