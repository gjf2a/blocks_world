mod operators;
mod methods;

#[cfg(test)]
mod tests {
    use crate::operators::{BlockState, BlockGoals};
    use anyhop::{find_first_plan, Task, Atom};
    use crate::methods::BlockMethod;

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum Block {B(char)}
    impl Atom for Block {}

    #[test]
    pub fn test1() {
        use Block::B;
        let state = BlockState::from(vec![B('b'), B('c')], vec![(B('a'), B('b'))]).unwrap();
        let goal = BlockGoals::new(vec![(B('a'), B('b')), (B('b'), B('c'))]);
        let plan = find_first_plan(&state, &goal,
                                   &vec![Task::MethodTag(BlockMethod::MoveBlocks)], 1);
        println!("{:?}", plan.unwrap());
    }
}