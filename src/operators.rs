use std::collections::{BTreeSet, BTreeMap};
use anyhop::{Atom, Operator, Orderable};

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct BlockGoals<B:Atom> {
    stacks: BTreeMap<B,B>
}

impl <B:Atom> BlockGoals<B> {
    pub fn new(goals: Vec<(B,B)>) -> Self {
        let mut result = BlockGoals {stacks: BTreeMap::new()};
        for (top, bottom) in goals {
            result.stacks.insert(top, bottom);
        }
        result
    }

    pub fn get_pos(&self, block: B) -> BlockPos<B> {
        match self.stacks.get(&block) {
            Some(other) => BlockPos::On(*other),
            None => BlockPos::Table
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct BlockState<B:Atom> {
    stacks: BTreeMap<B,B>,
    table: BTreeSet<B>,
    clear: BTreeSet<B>,
    holding: Option<B>
}

impl <B:Atom> Orderable for BlockState<B> {}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub enum BlockPos<B:Atom> {
    On(B), Table
}

impl <B:Atom> BlockState<B> {
    pub fn new(blocks: Vec<B>) -> Self {
        let mut state = BlockState {stacks: BTreeMap::new(), table: BTreeSet::new(), clear: BTreeSet::new(), holding: None};
        for block in blocks {
            state.table.insert(block);
            state.clear.insert(block);
        }
        state
    }

    // The block pairs should be in the order that they are to be stacked.
    // So the bottom pair is first, the pair above it next, and so forth.
    // It returns None if it can't easily make a plan.
    pub fn from(table: Vec<B>, block_piles: Vec<(B,B)>) -> Option<Self> {
        let mut all_blocks = table;
        let mut piles: Vec<B> = block_piles.iter().map(|p| p.0).collect();
        all_blocks.append(&mut piles);
        let mut result = BlockState::new(all_blocks);
        for (top, bottom) in block_piles {
            if !(result.pick_up(top) & result.stack(top, bottom)) {
                return None
            }
        }
        Some(result)
    }

    pub fn all_blocks(&self) -> Vec<B> {
        let mut result = Vec::new();
        self.stacks.iter().map(|entry| result.push(*entry.0));
        self.table.iter().for_each(|b| result.push(*b));
        result
    }

    pub fn get_pos(&self, block: B) -> BlockPos<B> {
        match self.stacks.get(&block) {
            Some(on) => BlockPos::On(*on),
            None => BlockPos::Table
        }
    }

    pub fn get_holding(&self) -> Option<B> {
        self.holding
    }

    pub fn clear(&self, block: B) -> bool {
        self.clear.contains(&block)
    }

    pub fn pick_up(&mut self, block: B) -> bool {
        if self.holding == None && self.table.contains(&block) && self.clear.contains(&block) {
            self.holding = Some(block);
            self.table.remove(&block);
            self.clear.remove(&block);
            true
        } else {false}
    }

    pub fn put_down(&mut self, block: B) -> bool {
        if self.holding == Some(block) {
            self.clear.insert(block);
            self.table.insert(block);
            self.holding = None;
            true
        } else {false}
    }

    pub fn unstack(&mut self, a: B, b: B) -> bool {
        if self.holding == None && self.get_pos(a) == BlockPos::On(b) && self.clear.contains(&a) {
            self.holding = Some(a);
            self.clear.insert(b);
            self.clear.remove(&a);
            self.stacks.remove(&a);
            true
        } else {false}
    }

    pub fn stack(&mut self, a: B, b: B) -> bool {
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
pub enum BlockOperator<B:Atom> {
    PickUp(B), PutDown(B), Stack(B,B), Unstack(B,B)
}

impl <B:Atom> Atom for BlockOperator<B> {}

impl <B:Atom> Operator<BlockState<B>> for BlockOperator<B> {
    fn attempt_update(&self, state: &mut BlockState<B>) -> bool {
        use BlockOperator::*;
        match self {
            PickUp(block) => state.pick_up(*block),
            PutDown(block) => state.put_down(*block),
            Stack(b1, b2) => state.stack(*b1, *b2),
            Unstack(b1, b2) => state.unstack(*b1, *b2)
        }
    }
}