use std::collections::{BTreeSet, BTreeMap};
use anyhop::{Atom, Operator};
use crate::methods::BlockGoals;

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct BlockState {
    stacks: BTreeMap<usize,usize>,
    table: BTreeSet<usize>,
    clear: BTreeSet<usize>,
    holding: Option<usize>
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub enum BlockPos {
    On(usize), Table
}

impl BlockState {
    pub fn new(blocks: Vec<usize>) -> Self {
        let mut state = BlockState {stacks: BTreeMap::new(), table: BTreeSet::new(), clear: BTreeSet::new(), holding: None};
        for block in blocks {
            state.table.insert(block);
            state.clear.insert(block);
        }
        state
    }

    pub fn from(table: Vec<usize>, block_piles: Vec<(usize,usize)>) -> Self {
        let mut all_blocks = table;
        let mut piles: Vec<usize> = block_piles.iter().map(|p| p.0).collect();
        all_blocks.append(&mut piles);
        let mut result = BlockState::new(all_blocks);

        for (top, bottom) in block_piles {
            result.stacks.insert(top, bottom);
            result.clear.remove(&bottom);
            result.table.remove(&top);
        }

        result
    }

    pub fn all_blocks(&self) -> Vec<usize> {
        let mut result = Vec::new();
        self.stacks.iter().for_each(|entry| result.push(*entry.0));
        self.table.iter().for_each(|b| result.push(*b));
        result
    }

    pub fn get_pos(&self, block: usize) -> BlockPos {
        match self.stacks.get(&block) {
            Some(on) => BlockPos::On(*on),
            None => BlockPos::Table
        }
    }

    pub fn get_holding(&self) -> Option<usize> {
        self.holding
    }

    pub fn clear(&self, block: usize) -> bool {
        self.clear.contains(&block)
    }

    pub fn pick_up(&mut self, block: usize) -> bool {
        if self.holding == None && self.table.contains(&block) && self.clear.contains(&block) {
            self.holding = Some(block);
            self.table.remove(&block);
            self.clear.remove(&block);
            true
        } else {false}
    }

    pub fn put_down(&mut self, block: usize) -> bool {
        if self.holding == Some(block) {
            self.clear.insert(block);
            self.table.insert(block);
            self.holding = None;
            true
        } else {false}
    }

    pub fn unstack(&mut self, a: usize, b: usize) -> bool {
        if self.holding == None && self.get_pos(a) == BlockPos::On(b) && self.clear.contains(&a) {
            self.holding = Some(a);
            self.clear.insert(b);
            self.clear.remove(&a);
            self.stacks.remove(&a);
            true
        } else {false}
    }

    pub fn stack(&mut self, a: usize, b: usize) -> bool {
        if self.holding == Some(a) && self.clear.contains(&b) {
            self.holding = None;
            self.clear.remove(&b);
            self.clear.insert(a);
            self.stacks.insert(a, b);
            true
        } else {false}
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BlockOperator {
    PickUp(usize), PutDown(usize), Stack(usize,usize), Unstack(usize,usize)
}

impl Operator for BlockOperator {
    type S = BlockState;
    type C = usize;
    type G = BlockGoals;

    fn cost(&self, _: &BlockState, _: &BlockGoals) -> usize {
        1
    }

    fn zero_cost() -> usize {
        0
    }

    fn attempt_update(&self, state: &mut BlockState) -> bool {
        use BlockOperator::*;
        match self {
            PickUp(block) => state.pick_up(*block),
            PutDown(block) => state.put_down(*block),
            Stack(b1, b2) => state.stack(*b1, *b2),
            Unstack(b1, b2) => state.unstack(*b1, *b2)
        }
    }
}
