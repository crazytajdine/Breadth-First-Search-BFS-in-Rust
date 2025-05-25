# Rust BFS Graph Traversal

This project demonstrates a straightforward implementation of the Breadth-First Search (BFS) algorithm in Rust. My goal was to explore Rust’s ownership and borrowing model by working with a common graph traversal problem.

## Overview

- Graph representation: `HashMap<T, Vec<T>>`
- Traversal method: Breadth-First Search (BFS)
- Returns a list of visited nodes in the order they were encountered
- Supports any node type that implements `Eq`, `Hash`, and `Display`

## Why Rust

Rust’s strict rules around ownership and lifetimes can be challenging at first, but they help prevent common bugs such as null pointer dereferences or data races. By writing this BFS implementation, I gained hands-on experience with:

- Borrowing and lifetimes
- Generic programming with trait bounds
- Standard collections (`HashMap`, `VecDeque`)

## Example

```rust
use std::collections::{HashMap, VecDeque};

fn main() {
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![5]);
    graph.insert(4, vec![3]);

    let visited = bfs(&graph, &1);

    for node in visited {
        print!("{},", node);
    }
}
```

The output should be:
```
1,2,3,5,
```

## Getting Started

1. Install Rust from rustup.rs if you do not already have it.
2. Clone this repository:
   ```bash
   git clone https://github.com/your-username/rust-bfs-graph.git
   cd rust-bfs-graph
   ```
3. Build and run:
   ```bash
   cargo run
   ```

## Future Improvements

- Implement Depth-First Search (DFS) for comparison
- Add support for weighted graphs
- Provide an option to return owned node values instead of references
- Include basic benchmarks to compare performance

## Reflections

Working on this project helped solidify my understanding of Rust’s key features. Seeing how lifetimes and borrowing rules operate in practice was particularly valuable. I encountered some challenges when returning borrowed data, but overcoming them made the learning experience worthwhile.

Feel free to explore, suggest improvements, or use this code as a starting point for your own experiments with Rust.
