mod node;

pub use node::Node;

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
