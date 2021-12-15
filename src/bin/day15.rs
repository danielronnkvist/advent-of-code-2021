use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    coordinate: (usize, usize),
    value: i32,
}

impl Node {
    pub fn new(x: usize, y: usize, value: i32) -> Self {
        Self {
            coordinate: (x, y),
            value,
        }
    }
}

#[derive(Debug, Clone)]
struct Edge<'a> {
    distance: i32,
    to: &'a Node,
}

impl<'a> Edge<'a> {
    pub fn new(to: &'a Node) -> Self {
        Self {
            distance: to.value,
            to,
        }
    }
}

struct Visit<V> {
    node: V,
    distance: i32,
}
impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

fn djikstra<'a>(start: &'a Node, edges: &'a HashMap<Node, Vec<Edge>>) -> HashMap<&'a Node, i32> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit {
        node: start,
        distance: 0,
    });

    while let Some(Visit { node, distance }) = to_visit.pop() {
        if !visited.insert(node) {
            continue;
        }

        if let Some(neighbours) = edges.get(node) {
            for neighbour in neighbours {
                let new_distance = neighbour.distance + distance;
                let is_shorter = distances
                    .get(neighbour.to)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(neighbour.to, new_distance);
                    to_visit.push(Visit {
                        node: neighbour.to,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

fn main() {
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
    let mut dimensions = 0;
    advent_of_code::read_lines("inputs/day15.txt")
        .unwrap()
        .into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            let line = line.unwrap();
            dimensions = line.len();
            line.chars().enumerate().for_each(|(x, c)| {
                let value = c.to_string().parse::<i32>().unwrap();
                for dy in 0..5 {
                    for dx in 0..5 {
                        let mut value = value;
                        for _ in 0..(dx + dy) as i32 {
                            value += 1;
                            if value == 10 {
                                value = 1;
                            }
                        }
                        nodes.insert(
                            (dx * dimensions + x, dy * dimensions + y),
                            Node::new(dx * dimensions + x, dy * dimensions + y, value),
                        );
                    }
                }
            });
        });

    let mut edges: HashMap<Node, Vec<Edge>> = HashMap::new();
    for y in 0..dimensions * 5 {
        for x in 0..dimensions * 5 {
            if let Some(from) = nodes.get(&(x, y)) {
                let mut to_edges = vec![];
                if let Some(to) = nodes.get(&(x + 1, y)) {
                    to_edges.push(Edge::new(to));
                }
                if let Some(to) = nodes.get(&(x, y + 1)) {
                    to_edges.push(Edge::new(to));
                }
                if x > 0 {
                    if let Some(to) = nodes.get(&(x - 1, y)) {
                        to_edges.push(Edge::new(to));
                    }
                }
                if y > 0 {
                    if let Some(to) = nodes.get(&(x, y - 1)) {
                        to_edges.push(Edge::new(to));
                    }
                }
                edges.insert(*from, to_edges);
            }
        }
    }

    let distances = djikstra(nodes.get(&(0, 0)).unwrap(), &edges);

    let p1_corner = dimensions - 1;
    let p1 = distances
        .get(nodes.get(&(p1_corner, p1_corner)).unwrap())
        .unwrap();
    println!("Puzzle 1: {}", p1);

    let p2_corner = (dimensions * 5) - 1;
    let p2 = distances
        .get(nodes.get(&(p2_corner, p2_corner)).unwrap())
        .unwrap();
    println!("Puzzle 2: {}", p2);
}
