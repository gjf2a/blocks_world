mod operators;
mod methods;

#[cfg(test)]
mod tests {
    use crate::operators::{BlockState, BlockGoals};
    use anyhop::{find_first_plan, Task, Atom};
    use crate::methods::BlockMethod;

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum Block {A,B,C}
    impl Atom for Block {}

    #[test]
    pub fn test1() {
        use Block::*;
        let state = BlockState::from(vec![B, C], vec![(A, B)]).unwrap();
        let goal = BlockGoals::new(vec![(A, B), (B, C)]);
        let plan = find_first_plan(&state, &goal,
                                   &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 3);
        println!("{:?}", plan.unwrap());
    }
}