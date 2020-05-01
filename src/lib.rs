mod operators;
mod methods;

#[cfg(test)]
mod tests {
    use crate::operators::{BlockState, BlockGoals, BlockOperator, is_valid};
    use anyhop::{find_first_plan, Task, Atom, BacktrackPreference, BacktrackStrategy, AnytimePlanner};
    use crate::methods::BlockMethod;
    use Block::*;
    use BlockOperator::*;

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    pub enum Block {A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S}
    impl Atom for Block {}

    #[test]
    pub fn test1() {
        let start = BlockState::from(vec![B, C], vec![(A, B)]).unwrap();
        let goal = BlockGoals::new(vec![(A, B), (B, C)]);
        let plan = find_first_plan(&start, &goal,
                                   &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert_eq!(plan, vec![Unstack(A, B), PutDown(A), PickUp(B), Stack(B, C), PickUp(A), Stack(A, B)]);
        assert!(is_valid(&plan, &start, &goal));
    }

    #[test]
    pub fn test2() {
        let start = BlockState::from(vec![C, D], vec![(A, C), (B, D)]).unwrap();
        let goal = BlockGoals::new(vec![(B, C), (A, D)]);
        let plan = find_first_plan(&start, &goal, &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert!(is_valid(&plan, &start, &goal));
    }

    pub fn big_test_states() -> (BlockState<Block>, BlockGoals<Block>) {
        (BlockState::from(vec![B,F,M,O],vec![(C, B), (P, C), (Q, P), (R, Q), (S, R), (G, F), (H, G), (I, H), (L, M), (A, L), (N, O), (D, N), (E, D), (J, E), (K, J)]).unwrap(),
        BlockGoals::new(vec![(O, M), (M, H), (H, I), (I, D), (L, B), (B, C), (C, P), (P, K), (K, G), (G, F)]))
    }

    #[test]
    pub fn test3() {
        let (start, goal) = big_test_states();
        let plan = find_first_plan(&start, &goal, &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert!(is_valid(&plan, &start, &goal));
    }

    #[test]
    pub fn anytime_blocks() {
        use BacktrackStrategy::*; use BacktrackPreference::*;
        let (start, goal) = big_test_states();
        for strategy in vec![Alternate(LeastRecent), Steady(LeastRecent), Steady(MostRecent)] {
            for apply_cutoff in vec![false, true] {
                let outcome = AnytimePlanner::plan(&start, &goal, &vec![Task::MethodTag(BlockMethod::MoveBlocks)], None, strategy, &|p| p.len(), 1, apply_cutoff);
                println!("strategy: {:?}\napply_cutoff: {}\n{}", strategy, apply_cutoff, outcome.report())
            }
        }
    }
}