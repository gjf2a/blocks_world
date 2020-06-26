use crate::operators::{BlockState, BlockGoals};
use std::{io,fs};
use std::collections::BTreeMap;
use pddl_problem_parser::{Predicate, PddlProblem};

pub fn make_block_problem_from(pddl_file: &str) -> io::Result<(BlockState, BlockGoals)> {
    let contents = fs::read_to_string(pddl_file)?;
    let parsed = pddl_problem_parser::PddlParser::parse(contents.as_str())?;

    let objects = enumerate_objects(&parsed);
    let (table, stacks) = extract_state(&parsed,&objects);
    let goals = extract_goals(&parsed, &objects);

    Ok((BlockState::from(table, stacks), BlockGoals::new(goals)))
}

fn enumerate_objects(parsed: &PddlProblem) -> BTreeMap<String,usize> {
    let mut objects = BTreeMap::new();
    for object in parsed.obj2type.keys() {
        objects.insert(String::from(object), objects.len());
    }
    objects
}

fn extract_state(parsed: &PddlProblem, objects: &BTreeMap<String,usize>) -> (Vec<usize>, Vec<(usize,usize)>) {
    let mut table = Vec::new();
    let mut stacks = Vec::new();
    for pred in parsed.bool_state.iter() {
        if pred.get_tag() == "ontable" {
            table.push(*objects.get(pred.get_arg(0)).unwrap());
        } else if pred.get_tag() == "on" {
            stacks.push(decode_on(&pred, &objects));
        }
    }
    (table, stacks)
}

fn extract_goals(parsed: &PddlProblem, objects: &BTreeMap<String,usize>) -> Vec<(usize,usize)> {
    let mut goals = Vec::new();
    for goal in parsed.goals.iter() {
        goals.push(decode_on(&goal, &objects));
    }
    goals
}

fn decode_on(p: &Predicate, objects: &BTreeMap<String,usize>) -> (usize, usize) {
    let top = obj_get(p, objects, 0);
    let bottom = obj_get(p, objects, 1);
    (top, bottom)
}

fn obj_get(p: &Predicate, objects: &BTreeMap<String,usize>, i: usize) -> usize {
    *objects.get(p.get_arg(i)).unwrap()
}
