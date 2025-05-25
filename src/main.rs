use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    hash::Hash,
};

type Graph<T> = HashMap<T, Vec<T>>;

fn bfs<'a, T: Eq + Hash + Display>(tree: &'a Graph<T>, start: &'a T) -> Vec<&'a T> {
    let mut visited = Vec::<&'a T>::new();

    let mut queue = VecDeque::<&T>::new();

    queue.push_back(start);

    while let Some(step) = queue.pop_front() {
        visited.push(step);

        if let Some(dests) = tree.get(step) {
            let nextdests: Vec<&T> = dests
                .iter()
                .filter(|dest| !visited.contains(dest) && !queue.contains(dest))
                .collect();

            for dest in nextdests {
                queue.push_back(dest);
            }
        }
    }

    visited
}

fn main() {
    let mut v = Graph::new();

    v.insert(1, vec![2, 3]);
    v.insert(2, vec![3]);
    v.insert(3, vec![5]);
    v.insert(4, vec![3]);

    let visited = bfs(&v, &1);

    for vis in visited {
        print!("{},", vis)
    }
    println!()
}
