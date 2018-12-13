use regex::Regex;
use petgraph::prelude::*;
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
    // let mut frontier: Vec<_> = graph.neighbors_directed(start_node, Direction::Outgoing).collect();
    let mut completed = Vec::new();

    while !frontier.is_empty() {
        frontier.sort_by(|&a, &b| { *&graph[a].cmp(&graph[b]) });
        println!("{:?}", frontier.iter().map(|&n| {*&graph[n]}).collect::<Vec<_>>());
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

struct Worker {
    task: char,
    remaining_time: i32,
}

impl Worker {
    
}

fn time_workers(graph: Graph<char, ()>) -> i32 {
    unimplemented!()
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