mod node;

pub use node::Node;

pub fn find_best_move(noeud: &Node, player: i32) -> usize {
    let empty_index = noeud.find_empty_index();
    if empty_index.len() == 1 {
        return empty_index[0];
    } else {
        let mut best_move_index = empty_index[0];
        let best_move_node = noeud.create_successor(best_move_index);
        let mut best_move_eval = minimax(&best_move_node, 9, player);
        for (_, val) in empty_index[1..].iter().enumerate() {
            let current_move_node = noeud.create_successor(*val);
            let current_move_eval = minimax(&current_move_node, 9, player);
            if current_move_eval > best_move_eval {
                best_move_eval = current_move_eval;
                best_move_index = *val;
            }
        }
        return best_move_index;
    }
}

fn minimax(noeud: &Node, depth: i32, player: i32) -> i32 {
    if depth == 0 || noeud.is_terminal() {
        return noeud.eval(player);
    }
    if noeud.is_maximizing_player(player) {
        let mut val: i32 = i32::MIN;
        for x in noeud.get_successors() {
            val = i32::max(val, minimax(&x, depth - 1, player))
        }
        return val;
    } else {
        let mut val: i32 = i32::MAX;
        for x in noeud.get_successors() {
            val = i32::min(val, minimax(&x, depth - 1, player))
        }
        return val;
    }
}
