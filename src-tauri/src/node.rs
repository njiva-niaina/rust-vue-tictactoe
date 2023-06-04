use std::collections::HashMap;

use crate::minimax;

#[derive(Debug, serde::Serialize)]
pub struct Node {
    tab: [i32; 9],
    player: i32,
    score: HashMap<i32, i32>,
    game_counter: i32,
}

impl Node {
    pub fn new() -> Self {
        Self {
            tab: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            player: 1,
            score: HashMap::from([(1, 0), (-1, 0)]),
            game_counter: 0,
        }
    }

    fn from(source: &Self) -> Self {
        Self {
            tab: source.tab,
            player: source.player,
            score: source.score.clone(),
            game_counter: 0,
        }
    }

    pub fn reset(&mut self) -> Self {
        self.tab = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        self.player = 1;
        self.score = self.score.clone();
        self.game_counter += 1;
        Self {
            tab: self.tab,
            player: self.player,
            score: self.score.clone(),
            game_counter: self.game_counter,
        }
    }

    pub fn make_move(&mut self, idx: usize) -> HashMap<String, i32> {
        if self.is_terminal() {
            return HashMap::from([
                (String::from("isGameOver"), 1),
                (String::from("isWinner"), 0),
                (String::from("player"), 0),
            ]);
        }

        let value = self.player;
        self.tab[idx] = value;

        if self.eval(value) == 100 {
            *self.score.get_mut(&value).unwrap() += 1;
            return HashMap::from([
                (String::from("isGameOver"), 1),
                (String::from("isWinner"), 1),
                (String::from("player"), value),
            ]);
        }

        if self.is_full() {
            return HashMap::from([
                (String::from("isGameOver"), 1),
                (String::from("isWinner"), 0),
                (String::from("player"), value),
            ]);
        }

        self.player *= -1;

        return HashMap::from([
            (String::from("isGameOver"), 0),
            (String::from("isWinner"), 0),
            (String::from("player"), value),
        ]);
    }

    pub fn create_successor(&self, idx: usize) -> Self {
        let mut new_node = Self::from(self);
        new_node.tab[idx] = new_node.player;
        new_node.player = -new_node.player;
        return new_node;
    }

    pub fn get_successors(&self) -> Vec<Self> {
        let mut successors: Vec<Self> = Vec::new();
        for (idx, val) in self.tab.iter().enumerate() {
            if *val == 0 {
                successors.push(self.create_successor(idx));
            }
        }
        return successors;
    }

    pub fn is_full(&self) -> bool {
        for val in self.tab.iter() {
            if *val == 0 {
                return false;
            }
        }
        return true;
    }

    pub fn is_terminal(&self) -> bool {
        if self.eval(-1) != 0 {
            return true;
        }
        if self.is_full() {
            return true;
        }
        return false;
    }

    pub fn is_maximizing_player(&self, player: i32) -> bool {
        self.player == player
    }

    pub fn find_empty_index(&self) -> Vec<usize> {
        let mut empty_index: Vec<usize> = Vec::new();
        for (idx, el) in self.tab.iter().enumerate() {
            if *el == 0 {
                empty_index.push(idx);
            }
        }
        return empty_index;
    }

    pub fn eval(&self, player: i32) -> i32 {
        if self.tab[0] != 0 && self.tab[0] == self.tab[1] && self.tab[1] == self.tab[2] {
            return player * self.tab[0] * 100;
        }
        if self.tab[3] != 0 && self.tab[3] == self.tab[4] && self.tab[4] == self.tab[5] {
            return player * self.tab[3] * 100;
        }
        if self.tab[6] != 0 && self.tab[6] == self.tab[7] && self.tab[7] == self.tab[8] {
            return player * self.tab[6] * 100;
        }

        if self.tab[0] != 0 && self.tab[0] == self.tab[3] && self.tab[3] == self.tab[6] {
            return player * self.tab[0] * 100;
        }
        if self.tab[1] != 0 && self.tab[1] == self.tab[4] && self.tab[4] == self.tab[7] {
            return player * self.tab[1] * 100;
        }
        if self.tab[2] != 0 && self.tab[2] == self.tab[5] && self.tab[5] == self.tab[8] {
            return player * self.tab[2] * 100;
        }

        if self.tab[0] != 0 && self.tab[0] == self.tab[4] && self.tab[4] == self.tab[8] {
            return player * self.tab[0] * 100;
        }
        if self.tab[2] != 0 && self.tab[2] == self.tab[4] && self.tab[4] == self.tab[6] {
            return player * self.tab[2] * 100;
        }

        return 0;
    }

    pub fn find_best_move(&self, player: i32) -> usize {
        let empty_index = self.find_empty_index();
        if empty_index.len() == 1 {
            return empty_index[0];
        } else {
            let mut best_move_index = empty_index[0];
            let best_move_node = self.create_successor(best_move_index);
            let mut best_move_eval = minimax(&best_move_node, 9, player);
            for (_, val) in empty_index[1..].iter().enumerate() {
                let current_move_node = self.create_successor(*val);
                let current_move_eval = minimax(&current_move_node, 9, player);
                if current_move_eval > best_move_eval {
                    best_move_eval = current_move_eval;
                    best_move_index = *val;
                }
            }
            return best_move_index;
        }
    }
}
