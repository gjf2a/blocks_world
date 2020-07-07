pub mod operators;
pub mod methods;
pub mod pddl_parser;

#[cfg(test)]
mod tests {
    use crate::operators::{BlockState, BlockOperator};
    use anyhop::{find_first_plan, Task, BacktrackPreference, BacktrackStrategy, AnytimePlannerBuilder, Goal};
    use crate::methods::{BlockGoals, BlockMethod};
    use BlockOperator::*;
    use crate::pddl_parser::make_block_problem_from;

    #[test]
    pub fn test1() {
        let start = BlockState::from(vec![1, 2], vec![(0, 1)]);
        let goal = BlockGoals::new(vec![(0, 1), (1, 2)]);
        let plan = find_first_plan(&start, &goal,
                                   &vec![Task::Method(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert_eq!(plan, vec![Unstack(0, 1), PutDown(0), PickUp(1), Stack(1, 2), PickUp(0), Stack(0, 1)]);
        assert!(goal.plan_valid(&start, &plan));
    }

    #[test]
    pub fn test2() {
        let start = BlockState::from(vec![2, 3], vec![(0, 2), (1, 3)]);
        let goal = BlockGoals::new(vec![(1, 2), (0, 3)]);
        let plan = find_first_plan(&start, &goal, &vec![Task::Method(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert!(goal.plan_valid(&start, &plan));
    }

    pub fn big_test_states() -> (BlockState, BlockGoals) {
        (BlockState::from(vec![1,5,12,14],vec![(2, 1), (15, 2), (16, 15), (17, 16), (18, 17), (6, 5), (7, 6),
                                               (8, 7), (11, 12), (0, 11), (13, 14), (3, 13), (4, 3), (9, 4), (10, 9)]),
        BlockGoals::new(vec![(14, 12), (12, 7), (7, 8), (8, 3), (11, 1), (1, 2), (2, 15), (15, 10), (10, 6), (6, 5)]))
    }

    #[test]
    pub fn test3() {
        let (start, goal) = big_test_states();
        let plan = find_first_plan(&start, &goal, &vec![Task::Method(BlockMethod::MoveBlocks)], 3).unwrap();
        println!("{:?}", plan);
        assert!(goal.plan_valid(&start, &plan));
    }

    #[test]
    pub fn test_pddl_4_0() {
        let (start, goal) = make_block_problem_from("probBLOCKS-4-0.pddl").unwrap();
        assert_eq!(start, BlockState::from(vec![2, 0, 1, 3], vec![]));
        assert_eq!(goal, BlockGoals::new(vec![(3, 2), (2, 1), (1, 0)]));
    }

    #[test]
    pub fn test_pddl_4_1() {
        let (start, goal) = make_block_problem_from("probBLOCKS-4-1.pddl").unwrap();
        assert_eq!(start, BlockState::from(vec![3], vec![(1, 2), (2, 0), (0, 3)]));
        assert_eq!(goal, BlockGoals::new(vec![(3, 2), (2, 0), (0, 1)]));
    }

    #[test]
    pub fn anytime_blocks() {
        use BacktrackStrategy::*; use BacktrackPreference::*;
        let (start, goal) = big_test_states();
        for strategy in vec![Alternate(LeastRecent), Steady(LeastRecent), Steady(MostRecent)] {
            for apply_cutoff in vec![false, true] {
                let outcome = AnytimePlannerBuilder::state_goal(&start, &goal)
                    .apply_cutoff(apply_cutoff)
                    .strategy(strategy)
                    .verbose(1)
                    .construct();
                println!("strategy: {:?}\napply_cutoff: {}\n{}", strategy, apply_cutoff, outcome.instance_csv());
            }
        }
    }
}