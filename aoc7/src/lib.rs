#[macro_use]
extern crate lazy_static;

use aoc_common;
use aoc_common::GenericResult;
use regex::Regex;
use aoc_common::GenericError;
use std::rc::Rc;
use std::collections::{HashSet, BinaryHeap, VecDeque};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Weak;
use std::cmp::Ordering;
use lazy_static::initialize;

#[derive(Debug, PartialEq)]
pub struct Dependency(char, char);

pub fn read_dependencies(path: &str) -> GenericResult<Vec<Dependency>> {
    let raw_input = aoc_common::read_raw_file_content(path)?;

    raw_input.iter().map(|input| parse_dependency(input)).collect()
}

fn parse_dependency(input: &str) -> GenericResult<Dependency> {
    lazy_static! {
        static ref LOG_REGEX: Regex = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    }

    LOG_REGEX
        .captures(input)
        .map(|captures| {
            internal_parse_dependency(&captures)
        })
        .ok_or(GenericError::new(&format!("Could not parse input: {}", input)).into())
        .and_then(|r| r)
}

fn internal_parse_dependency(captures: &regex::Captures) -> GenericResult<Dependency> {
    let source = parse_from_capture(captures, 1);
    let target = parse_from_capture(captures, 2);

    source.and_then(|s| target.map(|t| Dependency(s, t)))
}

fn parse_from_capture(captures: &regex::Captures, index: usize) -> GenericResult<char> {
    captures.get(index)
        .and_then(|m| m.as_str().chars().next())
        .ok_or(GenericError::new(&format!("Could not find dependency {}", index)).into())
}

#[derive(Debug, PartialEq)]
pub struct DependencyGraph {
    vertices: HashMap<char, Rc<DependencyVertex>>,
}

#[derive(Eq, PartialEq, Debug)]
struct Candidate(char);

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        let &Candidate(other_char) = other;
        let Candidate(this_char) = self;

        other_char.cmp(this_char)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Candidate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time(pub usize, pub usize);

#[derive(Debug, PartialEq)]
pub struct WorkAssignment(pub char, pub usize, pub Time);

#[derive(Eq, PartialEq, Debug)]
struct OngoingWork(char, usize, Time);

impl Ord for OngoingWork {
    fn cmp(&self, other: &Self) -> Ordering {
        let OngoingWork(_, _, Time(_, end_time)) = self;
        let &OngoingWork(_, _, Time(_, other_end_time)) = other;

        other_end_time.cmp(end_time)
    }
}

impl PartialOrd for OngoingWork {
    fn partial_cmp(&self, other: &OngoingWork) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl DependencyGraph {
    pub fn new(vertices: HashMap<char, Rc<DependencyVertex>>) -> DependencyGraph {
        DependencyGraph {
            vertices,
        }
    }

    pub fn generate_graph(dependencies: &Vec<Dependency>) -> DependencyGraph {
        let vertices: HashMap<char, Rc<DependencyVertex>> = dependencies
            .iter()
            .flat_map(|&Dependency(source, target)| vec![source, target])
            .collect::<HashSet<char>>()
            .into_iter()
            .map(|vertex| (vertex, Rc::new(DependencyVertex::new(vertex))))
            .collect();

        for Dependency(source, target) in dependencies {
            let source = vertices.get(source).unwrap();
            let target = vertices.get(target).unwrap();

            source.add_outgoing(Rc::clone(target));
            target.add_incoming(Rc::clone(source));
        }

        DependencyGraph {
            vertices,
        }
    }

    pub fn sort_topologically(&self) -> Vec<char> {
        let mut unfulfilled_dependencies: HashMap<char, usize> = HashMap::with_capacity(self.vertices.len());
        let mut priority_heap: BinaryHeap<Candidate> = BinaryHeap::new();
        let mut result = Vec::with_capacity(self.vertices.len());

        self.initialize_dependencies(&mut unfulfilled_dependencies, &mut priority_heap);

        while let Some(Candidate(next)) = priority_heap.pop() {
            result.push(next);

            self.complete_requirement(&mut unfulfilled_dependencies, &mut priority_heap, next)
        }

        result
    }

    fn complete_requirement(&self, unfulfilled_dependencies: &mut HashMap<char, usize>, priority_heap: &mut BinaryHeap<Candidate>, next: char) {
        let vertex = self.vertices.get(&next).unwrap();
        for outgoing in vertex.outgoing.borrow().keys() {
            if let Some(value) = unfulfilled_dependencies.get_mut(outgoing) {
                *value -= 1;

                if *value == 0 {
                    unfulfilled_dependencies.remove(outgoing);
                    priority_heap.push(Candidate(*outgoing));
                }
            }
        }
    }

    fn initialize_dependencies(&self, unfulfilled_dependencies: &mut HashMap<char, usize>, priority_heap: &mut BinaryHeap<Candidate>) {
        for dependency_vertex in self.vertices.values() {
            if dependency_vertex.incoming.borrow().len() == 0 {
                priority_heap.push(Candidate(dependency_vertex.id));
            } else {
                unfulfilled_dependencies.insert(dependency_vertex.id, dependency_vertex.incoming.borrow().len());
            }
        }
    }

    pub fn assign_work(&self, num_workers: usize, constant_work_time: usize) -> Vec<WorkAssignment> {
        let mut current_time = 0;
        let mut free_workers = (0..num_workers).collect::<VecDeque<usize>>();
        let mut work_items: BinaryHeap<Candidate> = BinaryHeap::new();
        let mut dependencies = HashMap::with_capacity(self.vertices.len());
        let mut current_work: BinaryHeap<OngoingWork> = BinaryHeap::new();
        let mut result = Vec::with_capacity(self.vertices.len());

        self.initialize_dependencies(&mut dependencies, &mut work_items);

        while !current_work.is_empty() || !work_items.is_empty() {
            // 1. assign work
            while let Some(&Candidate(work_item)) = work_items.peek() {
                if let Some(free_worker) = free_workers.pop_front() {
                    current_work.push(OngoingWork(work_item, free_worker, Time(current_time, current_time + DependencyGraph::required_time(work_item, constant_work_time))));
                    work_items.pop();
                } else {
                    break;
                }
            }

            // 2. advance time to next finished work item
            if let Some(OngoingWork(finished_work, worker, Time(start, end_time))) = current_work.pop() {
                current_time = end_time;

                result.push(WorkAssignment(finished_work, worker, Time(start, end_time)));
                free_workers.push_front(worker);

                self.complete_requirement(&mut dependencies, &mut work_items, finished_work);
            }
        }

        result
    }

    fn required_time(work_item: char, constant_work_time: usize) -> usize {
        (work_item as usize - 'A' as usize + 1) + constant_work_time
    }
}

#[derive(Debug)]
pub struct DependencyVertex {
    id: char,
    incoming: RefCell<HashMap<char, Weak<DependencyVertex>>>,
    outgoing: RefCell<HashMap<char, Rc<DependencyVertex>>>,
}

impl DependencyVertex {
    pub fn new(id: char) -> DependencyVertex {
        DependencyVertex {
            id,
            incoming: RefCell::new(HashMap::new()),
            outgoing: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_incoming(&self, source: Rc<DependencyVertex>) {
        let source_id = source.id;
        self.incoming.borrow_mut().insert(source_id, Rc::downgrade(&source));
    }

    pub fn add_outgoing(&self, target: Rc<DependencyVertex>) {
        let target_id = target.id;
        self.outgoing.borrow_mut().insert(target_id, target);
    }
}

impl PartialEq for DependencyVertex {
    fn eq(&self, other: &DependencyVertex) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dependency() {
        assert_eq!(parse_dependency("Step Z must be finished before step N can begin.").unwrap(), Dependency('Z', 'N'));
    }

    #[test]
    fn test_dependency_graph_generation() {
        let source = Rc::new(DependencyVertex::new('A'));
        let target = Rc::new(DependencyVertex::new('B'));

        source.add_outgoing(Rc::clone(&target));
        target.add_incoming(Rc::clone(&source));

        let vertices = vec![source, target]
            .into_iter()
            .map(|vertex| {
                (vertex.id, vertex)
            })
            .collect::<HashMap<char, Rc<DependencyVertex>>>();

        let dependency_graph = DependencyGraph::new(vertices);

        let dependencies = vec![Dependency('A', 'B')];
        let generated_graph = DependencyGraph::generate_graph(&dependencies);

        assert_eq!(generated_graph, dependency_graph);
    }

    #[test]
    fn test_sort_topologically() {
        let dependencies =  vec![Dependency('A', 'B'), Dependency('C', 'B'), Dependency('B', 'D')];
        let graph = DependencyGraph::generate_graph(&dependencies);
        assert_eq!(graph.sort_topologically(), vec!['A', 'C', 'B', 'D']);
    }

    #[test]
    fn test_work_assignment() {
        let dependencies = read_dependencies("test_input.txt").unwrap();
        let graph = DependencyGraph::generate_graph(&dependencies);

        assert_eq!(graph.assign_work(2, 0), vec![
            WorkAssignment('C', 0, Time(0, 3)),
            WorkAssignment('A', 0, Time(3, 4)),
            WorkAssignment('B', 0, Time(4, 6)),
            WorkAssignment('F', 1, Time(3, 9)),
            WorkAssignment('D', 0, Time(6, 10)),
            WorkAssignment('E', 0, Time(10, 15))])
    }
}
