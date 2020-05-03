#[macro_use]
extern crate io_error;

pub mod operators;
pub mod methods;
pub mod pddl_parser;

#[cfg(test)]
mod tests {
    use crate::operators::{BlockState, BlockGoals, BlockOperator, is_valid};
    use anyhop::{find_first_plan, Task, Atom, BacktrackPreference, BacktrackStrategy, AnytimePlanner};
    use crate::methods::BlockMethod;
    use Block::*;
    use BlockOperator::*;
    use crate::pddl_parser::make_block_problem_from;
    use crate::pddl_parser::B;

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    pub enum Block {A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S}
    impl Atom for Block {}

    #[test]
    pub fn test1() {
        let start = BlockState::from(vec![B, C], vec![(A, B)]);
        let goal = BlockGoals::new(vec![(A, B), (B, C)]);
        let plan = find_first_plan(&start, &goal,
                                   &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert_eq!(plan, vec![Unstack(A, B), PutDown(A), PickUp(B), Stack(B, C), PickUp(A), Stack(A, B)]);
        assert!(is_valid(&plan, &start, &goal));
    }

    #[test]
    pub fn test2() {
        let start = BlockState::from(vec![C, D], vec![(A, C), (B, D)]);
        let goal = BlockGoals::new(vec![(B, C), (A, D)]);
        let plan = find_first_plan(&start, &goal, &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert!(is_valid(&plan, &start, &goal));
    }

    pub fn big_test_states() -> (BlockState<Block>, BlockGoals<Block>) {
        (BlockState::from(vec![B,F,M,O],vec![(C, B), (P, C), (Q, P), (R, Q), (S, R), (G, F), (H, G), (I, H), (L, M), (A, L), (N, O), (D, N), (E, D), (J, E), (K, J)]),
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
    pub fn test_pddl_4_0() {
        let (start, goal) = make_block_problem_from("probBLOCKS-4-0.pddl").unwrap();
        assert_eq!(start, BlockState::from(vec![B::B(2), B::B(0), B::B(1), B::B(3)], vec![]));
        assert_eq!(goal, BlockGoals::new(vec![(B::B(0), B::B(3)), (B::B(1), B::B(2)), (B::B(3), B::B(1))]));
    }

    #[test]
    pub fn test_pddl_4_1() {
        let (start, goal) = make_block_problem_from("probBLOCKS-4-1.pddl").unwrap();
        assert_eq!(start, BlockState::from(vec![B::B(2)], vec![(B::B(0), B::B(2)), (B::B(1), B::B(0)), (B::B(3), B::B(1))]));
        assert_eq!(goal, BlockGoals::new(vec![(B::B(0), B::B(3)), (B::B(1), B::B(0)), (B::B(2), B::B(1))]));
    }

    #[test]
    pub fn anytime_blocks() {
        use BacktrackStrategy::*; use BacktrackPreference::*;
        let (start, goal) = big_test_states();
        for strategy in vec![Alternate(LeastRecent), Steady(LeastRecent), Steady(MostRecent)] {
            for apply_cutoff in vec![false, true] {
                let outcome: AnytimePlanner<BlockState<Block>, BlockGoals<Block>, BlockOperator<Block>, BlockMethod<Block>, BlockMethod<Block>, usize> = AnytimePlanner::plan(&start, &goal, None, strategy, &|p| p.len(), 1, apply_cutoff);
                println!("strategy: {:?}\napply_cutoff: {}\n{}", strategy, apply_cutoff, outcome.report())
            }
        }
    }
}