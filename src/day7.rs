use regex::Regex;
use petgraph::prelude::*;
#[allow(unused_imports)]
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Dep(char, char);

fn parse_input(input: &str) -> Vec<Dep> {
    let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    input.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Dep(caps.get(1).unwrap().as_str().chars().next().unwrap(), caps.get(2).unwrap().as_str().chars().next().unwrap())
        })
        .collect()
}

fn build_graph(deps: Vec<Dep>) -> Graph<char, ()> {
    let mut graph = Graph::<char, ()>::new();
    let mut node_map = HashMap::<char, NodeIndex>::new();

    // TODO: could have used GraphMap for building
    let edges = deps.into_iter()
        .map(|Dep(a, b)| {
            let node_a = *node_map.entry(a).or_insert_with(|| { graph.add_node(a) });

            let node_b = *node_map.entry(b).or_insert_with(|| { graph.add_node(b) });

            (node_a, node_b)
        }).collect::<Vec<_>>();
    
    graph.extend_with_edges(edges);

    graph
}

fn all_dependencies_completed(idx: NodeIndex, graph: &Graph<char, ()>, completed: &Vec<NodeIndex>) -> bool {
    graph.neighbors_directed(idx, Direction::Incoming).all(|dep| { completed.contains(&dep) })  
}

fn resolve_order(graph: Graph<char, ()>) -> Vec<char> {
    let mut frontier: Vec<_> = graph.externals(Direction::Incoming).collect();
    let mut completed = Vec::new();

    while !frontier.is_empty() {
        frontier.sort_by(|&a, &b| { *&graph[a].cmp(&graph[b]) });
        // println!("{:?}", frontier.iter().map(|&n| {*&graph[n]}).collect::<Vec<_>>());
        let (i, &next) = frontier.iter().enumerate().filter(|(_, &node)| { all_dependencies_completed(node, &graph, &completed)}).next().unwrap();
        frontier.swap_remove(i);
        completed.push(next);
        let mut expand: Vec<_> = graph.neighbors_directed(next, Direction::Outgoing).filter(|node| { !completed.contains(node) && !frontier.contains(node)}).collect();
        frontier.append(&mut expand);
    }

    return completed.into_iter().map(|node| {
        *&graph[node]
    }).collect();
}

#[derive(Debug, Copy, Clone)]
struct Worker {
    task_node: Option<NodeIndex>,
    remaining_time: i32,
}

impl Worker {
    fn new() -> Worker {
        Worker {
            task_node: None,
            remaining_time: -1
        }
    }

    fn assign(&mut self, task: NodeIndex, time: i32) {
        self.task_node = Some(task);
        self.remaining_time = time;
    }

    fn complete(&mut self) {
        self.task_node = None;
        self.remaining_time = -1;
    }

    fn decrement_time(&mut self) {
        self.remaining_time -= 1;
    }
    
}

fn time_workers(graph: Graph<char, ()>) -> i32 {
    let mut frontier: Vec<_> = graph.externals(Direction::Incoming).collect();
    let mut completed = Vec::<NodeIndex>::new();
    let mut seconds_elapsed = 0;
    let mut workers = vec![Worker::new(); 5];

    loop {
        frontier.sort_by(|&a, &b| { *&graph[a].cmp(&graph[b]) });

        // collect completed tasks
        workers.iter_mut()
            .filter(|w| { w.remaining_time == 0 })
            .for_each(|w| {
                let next = w.task_node.unwrap();
                completed.push(next);
                let mut expand: Vec<_> = graph.neighbors_directed(next, Direction::Outgoing)
                    .filter(|node| { !completed.contains(node) && !frontier.contains(node)})
                    .collect();
                frontier.append(&mut expand);
                w.complete();
            });

        // done, break now so we don't count an extra second
        if completed.len() == graph.node_count() {
            break;
        }

        // println!("{:?}", frontier.iter().map(|&n| {*&graph[n]}).collect::<Vec<_>>());
        // assign available tasks to workers
        let next_tasks = frontier.iter()
            .filter(|&&node| { all_dependencies_completed(node, &graph, &completed)})
            .zip(workers.iter_mut().filter(|w| { w.task_node.is_none() }));

        let mut nodes_to_remove = Vec::new();

        for (&node, worker) in next_tasks {
            let time = 60 + *&graph[node] as i32 - 'A' as i32 + 1;
            worker.assign(node, time);
            nodes_to_remove.push(node);
        }

        frontier.retain(|n| { !nodes_to_remove.contains(n) });

        workers.iter_mut()
            .filter(|w| w.task_node.is_some())
            .for_each(|w| w.decrement_time());

        seconds_elapsed += 1;
    }

    seconds_elapsed
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    let deps = parse_input(input);
    let graph = build_graph(deps);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let solution = resolve_order(graph);
    String::from_iter(solution)
    // println!("{:?}", solution);
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
    let deps = parse_input(input);
    let graph = build_graph(deps);
    time_workers(graph)
}