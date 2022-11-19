// 电脑玩家
use crate::Step;
use crate::StoneMap;
use crate::stonemap::StoneIndex;

pub struct ComputerPlayer {
    map: StoneMap,
    max_depth: i32,
    history_table: [[i32; 90]; 90],
}

impl ComputerPlayer {
    pub fn new(map: StoneMap, max_depth: i32) -> Self {
        ComputerPlayer {
            map,
            max_depth,
            history_table: [[0; 90]; 90],
        }
    }

    pub fn play(&mut self) -> Step {
        let mut best_steps:Vec<Step> = Vec::new();
        let possible_steps = self.map.generate_stone_steps();

        let mut alpha = -0x3f3f3f3f - self.max_depth - 1;
        let beta = 0x3f3f3f3f + self.max_depth + 1;
        
        for step in possible_steps.iter() {
            self.map.make_move(step);
            let value = self.get_min(self.max_depth-1, alpha-1, beta);
            self.map.revoke_move(step);

            if value > alpha {
                assert!(alpha < beta);
                alpha = value;
                best_steps.clear();
                best_steps.push(*step);
            } else if value == alpha {
                best_steps.push(*step);
            }
        }
        
        assert!(best_steps.len() > 0);
        best_steps[0]
    }

    fn get_min(&mut self, current_depth: i32, alpha: i32, beta: i32) -> i32 {
        let mut beta = beta;
        if current_depth <= 0 { return self.map.evaluate(); }
        else if let StoneIndex::Dead = self.map.up_stones[4] { return -0x3f3f3f3f - current_depth; }
        else if let StoneIndex::Dead = self.map.down_stones[4] { return 0x3f3f3f3f + current_depth; }

        let mut possible_steps = self.map.generate_stone_steps();
        for step in possible_steps.iter_mut() {
            step.weight = self.get_history_score(step);
        }

        possible_steps.sort_by(|step1, step2| {
            if step1.weight > step2.weight { std::cmp::Ordering::Less }
            else if step1.weight == step2.weight { std::cmp::Ordering::Equal }
            else { std::cmp::Ordering::Greater }
        });

        for step in possible_steps.iter() {
            self.map.make_move(step);
            let value = self.get_max(current_depth-1, alpha, beta);
            self.map.revoke_move(step);

            if value < beta {
                beta = value;
                if alpha >= beta {
                    self.add_history_score(step, current_depth);
                    return alpha;
                }
            }
        }
        return beta;
    }

    fn get_max(&mut self, current_depth: i32, alpha: i32, beta: i32) -> i32 {
        if current_depth <= 0 {  return self.map.evaluate(); }
        else if let StoneIndex::Dead = self.map.up_stones[4] { return -0x3f3f3f3f - current_depth; }
        else if let StoneIndex::Dead = self.map.down_stones[4] { return  0x3f3f3f3f + current_depth; }
        
        let mut alpha = alpha;
        let mut possible_steps = self.map.generate_stone_steps();
        for step in possible_steps.iter_mut() {
            step.weight = self.get_history_score(step);
        }   
        possible_steps.sort_by(|step1, step2| {
            if step1.weight > step2.weight { std::cmp::Ordering::Less }
            else if step1.weight == step2.weight { std::cmp::Ordering::Equal }
            else { std::cmp::Ordering::Greater }
        });

        for step in possible_steps.iter() {
            self.map.make_move(step);
            let value = self.get_min(current_depth-1, alpha, beta);
            self.map.revoke_move(step);
            if value > alpha {
                alpha = value;
                if alpha >= beta {
                    self.add_history_score(step, current_depth);
                    return beta;
                }
            }
        }
        alpha
    }

    fn get_history_score(&self, step: &Step) -> i32 {
        let n_from = step.from.0 * 9 + step.from.1;
        let n_dest = step.from.0 * 9 + step.from.1;
        self.history_table[n_from][n_dest]
    }

    fn add_history_score(&mut self, step:&Step, depth: i32) {
        let n_from = step.from.0 * 9 + step.from.1;
        let n_dest = step.from.0 * 9 + step.from.1;
        self.history_table[n_from][n_dest] += 1 << depth
    }
}
