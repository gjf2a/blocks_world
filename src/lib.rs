mod operators;
mod methods;

#[cfg(test)]
mod tests {
    use crate::operators::{BlockState, BlockGoals, BlockOperator, is_valid};
    use anyhop::{find_first_plan, Task, Atom};
    use crate::methods::BlockMethod;

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum Block {A,B,C}
    impl Atom for Block {}

    #[test]
    pub fn test1() {
        use Block::*; use BlockOperator::*;
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

    }
}