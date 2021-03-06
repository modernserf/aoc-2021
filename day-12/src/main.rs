use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Node {
    Start,
    End,
    Major([char; 2]),
    Minor([char; 2]),
}

type Graph = HashMap<Node, Vec<Node>>;

fn parse_node(str: &str) -> Node {
    match str {
        "start" => Node::Start,
        "end" => Node::End,
        _ => {
            let ch = str.chars().nth(0).unwrap();
            let ch2 = str.chars().nth(1).unwrap();
            match ch {
                'a'..='z' => Node::Minor([ch, ch2]),
                'A'..='Z' => Node::Major([ch, ch2]),
                _ => {
                    panic!("invalid char");
                }
            }
        }
    }
}

struct QueueFrame {
    current_node: Node,
    has_visited_twice: bool,
    visited_nodes: Rc<HashSet<Node>>,
}

fn find_paths(graph: &Graph, can_visit_twice: bool) -> usize {
    let mut completed_path_count = 0;
    let mut queue = VecDeque::new();

    queue.push_back(QueueFrame {
        current_node: Node::Start,
        visited_nodes: Rc::new(HashSet::new()),
        has_visited_twice: !can_visit_twice,
    });

    while let Some(frame) = queue.pop_front() {
        let next_node_options = graph.get(&frame.current_node).unwrap();
        for node in next_node_options {
            match node {
                Node::Start => {
                    // do nothing
                }
                Node::End => {
                    completed_path_count += 1;
                }
                Node::Major(_) => {
                    let next_frame = QueueFrame {
                        current_node: *node,
                        visited_nodes: Rc::clone(&frame.visited_nodes),
                        has_visited_twice: frame.has_visited_twice,
                    };
                    queue.push_back(next_frame);
                }
                Node::Minor(_) => {
                    let visited = frame.visited_nodes.contains(node);
                    if !visited {
                        let mut next_visited_nodes = (*frame.visited_nodes).clone();
                        next_visited_nodes.insert(*node);

                        let next_frame = QueueFrame {
                            current_node: *node,
                            visited_nodes: Rc::new(next_visited_nodes),
                            has_visited_twice: frame.has_visited_twice,
                        };
                        queue.push_back(next_frame);
                    } else if !frame.has_visited_twice {
                        let next_frame = QueueFrame {
                            current_node: *node,
                            visited_nodes: Rc::clone(&frame.visited_nodes),
                            has_visited_twice: true,
                        };
                        queue.push_back(next_frame);
                    }
                }
            }
        }
    }

    completed_path_count
}

fn main() {
    let input = include_str!("input.txt");
    let pairs = input
        .split("\n")
        .map(|line| {
            let splits = line.split("-").collect::<Vec<_>>();
            (parse_node(&splits[0]), parse_node(&splits[1]))
        })
        .collect::<Vec<_>>();
    let graph = {
        let mut graph: Graph = HashMap::new();
        for (from, to) in pairs {
            graph
                .entry(from)
                .and_modify(|list| {
                    list.push(to);
                })
                .or_insert(vec![to]);
            graph
                .entry(to)
                .and_modify(|list| {
                    list.push(from);
                })
                .or_insert(vec![from]);
        }
        graph
    };
    let paths = find_paths(&graph, false);
    println!("Part 1: {}", paths);
    let paths_2 = find_paths(&graph, true);
    println!("Part 2: {}", paths_2);
}
