use super::operators::*;
use anyhop::{Atom, Method, Task, MethodResult, Goal};
use std::collections::BTreeMap;

pub fn is_done(b1: usize, state: &BlockState, goal: &BlockGoals) -> bool {
    let pos = state.get_pos(b1);
    pos == goal.get_pos(b1) && match pos {
        BlockPos::On(b2) => is_done(b2, state, goal),
        BlockPos::Table => true
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub enum Status {
    Done(usize),
    Inaccessible(usize),
    Move(usize,BlockPos),
    Waiting(usize)
}

impl Status {
    pub fn new(b: usize, state: &BlockState, goal: &BlockGoals) -> Self {
        if is_done(b, state, goal) {
            Status::Done(b)
        } else if !state.clear(b) {
            Status::Inaccessible(b)
        } else {
            match goal.get_pos(b) {
                BlockPos::Table => Status::Move(b, BlockPos::Table),
                BlockPos::On(b2) => if is_done(b2, state, goal) && state.clear(b2) {
                    Status::Move(b, BlockPos::On(b2))
                } else {
                    Status::Waiting(b)
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub enum BlockMethod {
    MoveBlocks,
    MoveOne(usize, BlockPos),
    Get(usize),
    Put(BlockPos)
}

impl Method for BlockMethod {
    type S = BlockState;
    type G = BlockGoals;
    type O = BlockOperator;

    fn apply(&self, state: &BlockState, goal: &BlockGoals) -> MethodResult<BlockOperator, BlockMethod> {
        use BlockMethod::*;
        match self {
            MoveBlocks => move_blocks(state, goal),
            MoveOne(block, pos) => move_one(*block, *pos),
            Get(block) => get(state, *block),
            Put(pos) => put(state, *pos)
        }
    }
}

fn get_status(state: &BlockState, goal: &BlockGoals) -> Vec<Status> {
    state.all_blocks().iter().map(|b| Status::new(*b, state, goal)).collect()
}

fn move_blocks(state: &BlockState, goal: &BlockGoals) -> MethodResult<BlockOperator, BlockMethod> {
    use BlockMethod::*; use MethodResult::*; use Task::*;
    let status = get_status(state, goal);
    for stat in status.iter() {
        if let Status::Move(b, pos) = stat {
            return TaskLists(vec![vec![Method(MoveOne(*b, *pos)), Method(MoveBlocks)]])
        }
    }

    let waiting: Vec<Vec<Task<BlockOperator, BlockMethod>>> = status.iter()
        .filter_map(|s| match s {
            Status::Waiting(b) => Some(vec![Method(MoveOne(*b, BlockPos::Table)),Method(MoveBlocks)]),
            _ => None
        })
        .collect();
    TaskLists(waiting)
}

fn move_one(block: usize, pos: BlockPos) -> MethodResult<BlockOperator, BlockMethod> {
    use BlockMethod::*; use MethodResult::*; use Task::*;
    TaskLists(vec![vec![Method(Get(block)), Method(Put(pos))]])
}

fn get(state: &BlockState, block: usize) -> MethodResult<BlockOperator, BlockMethod> {
    use BlockOperator::*; use MethodResult::*; use Task::*; use BlockPos::*;
    if state.clear(block) {
        TaskLists(match state.get_pos(block) {
            Table => vec![vec![Operator(PickUp(block))]],
            On(block2) => vec![vec![Operator(Unstack(block, block2))]]
        })
    } else {
        Failure
    }
}

fn put(state: &BlockState, pos: BlockPos) -> MethodResult<BlockOperator, BlockMethod> {
    use BlockOperator::*; use MethodResult::*; use Task::*; use BlockPos::*;
    if let Some(b) = state.get_holding() {
        TaskLists(match pos {
            Table => vec![vec![Operator(PutDown(b))]],
            On(b2) => vec![vec![Operator(Stack(b, b2))]]
        })
    } else {
        Failure
    }
}

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct BlockGoals {
    stacks: BTreeMap<usize,usize>
}

impl BlockGoals {
    pub fn new(goals: Vec<(usize,usize)>) -> Self {
        let mut result = BlockGoals {stacks: BTreeMap::new()};
        for (top, bottom) in goals {
            result.stacks.insert(top, bottom);
        }
        result
    }

    pub fn get_pos(&self, block: usize) -> BlockPos {
        match self.stacks.get(&block) {
            Some(other) => BlockPos::On(*other),
            None => BlockPos::Table
        }
    }
}

impl Goal for BlockGoals {
    type O = BlockOperator;
    type M = BlockMethod;
    type S = BlockState;
    type C = usize;

    fn starting_tasks(&self) -> Vec<Task<BlockOperator, BlockMethod>> {
        vec![Task::Method(BlockMethod::MoveBlocks)]
    }

    fn accepts(&self, state: &Self::S) -> bool {
        self.stacks.iter()
            .all(|goal| state.get_pos(*goal.0) == BlockPos::On(*goal.1))
    }

    fn distance_from(&self, state: &Self::S) -> Self::C {
        let status = get_status(state, self);
        let num_done = status.iter()
            .filter(|s| match s {Status::Done(_) => true, _ => false})
            .count();
        status.len() - num_done
    }
}