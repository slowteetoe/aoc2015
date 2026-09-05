use std::collections::HashMap;

use petgraph::graphmap::UnGraphMap;

advent_of_code::solution!(9);

fn parse_input(input: &str) -> UnGraphMap<&str, u32> {
    let mut g = UnGraphMap::<_, _>::new();
    input
        .lines()
        .map(|line| {
            let (cities, dist) = line.split_once(" = ").unwrap();
            let dist = dist.parse::<u32>().unwrap();
            let (a, b) = cities.split_once(" to ").unwrap();
            (a, b, dist)
        })
        .for_each(|(a, b, dist)| {
            g.add_edge(a, b, dist);
        });
    g
}
fn shortest_path_all_nodes<'a>(graph: &UnGraphMap<&'a str, u32>) -> Option<(u32, Vec<&'a str>)> {
    let nodes: Vec<&str> = graph.nodes().collect();
    let n = nodes.len();
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some((0, vec![nodes[0]]));
    }

    // Map each node string slice to an integer index (0 to n-1) for bitmasking
    let node_to_idx: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, &node)| (node, i))
        .collect();

    // Adjacency matrix for quick weight lookup (u32::MAX means no edge)
    let mut adj = vec![vec![u32::MAX; n]; n];
    for (u, v, &weight) in graph.all_edges() {
        let idx_u = node_to_idx[&u];
        let idx_v = node_to_idx[&v];
        adj[idx_u][idx_v] = weight;
        adj[idx_v][idx_u] = weight; // Undirected
    }

    // DP table: dp[mask][current_node] = (min_cost, parent_node)
    // mask: a bitmask where the i-th bit is 1 if node i has been visited
    let num_states = 1 << n;
    let mut dp = vec![vec![(u32::MAX, None); n]; num_states];

    // Base cases: Base cost is 0 for starting at any single node
    for i in 0..n {
        dp[1 << i][i] = (0, None);
    }

    // Iterate through all bit combinations (masks)
    for mask in 1..num_states {
        for u in 0..n {
            if (mask & (1 << u)) == 0 || dp[mask][u].0 == u32::MAX {
                continue;
            }

            let current_cost = dp[mask][u].0;

            // Try to visit an unvisited neighbor `v`
            for v in 0..n {
                if (mask & (1 << v)) == 0 && adj[u][v] != u32::MAX {
                    let next_mask = mask | (1 << v);
                    let path_cost = current_cost + adj[u][v];

                    if path_cost < dp[next_mask][v].0 {
                        dp[next_mask][v] = (path_cost, Some(u));
                    }
                }
            }
        }
    }

    // Find the minimum cost state where all nodes are visited (mask = (1 << n) - 1)
    let final_mask = num_states - 1;
    let mut min_total_cost = u32::MAX;
    let mut last_node = None;

    for i in 0..n {
        if dp[final_mask][i].0 < min_total_cost {
            min_total_cost = dp[final_mask][i].0;
            last_node = Some(i);
        }
    }

    if min_total_cost == u32::MAX {
        return None; // No path connects all nodes
    }

    // Reconstruct the path backwards
    let mut path_indices = Vec::new();
    let mut curr_mask = final_mask;
    let mut curr_node = last_node;

    while let Some(u) = curr_node {
        path_indices.push(u);
        let prev_node = dp[curr_mask][u].1;
        curr_mask ^= 1 << u; // Clear the bit
        curr_node = prev_node;
    }
    path_indices.reverse();

    // Map indices back to original &str references
    let path_nodes = path_indices.into_iter().map(|idx| nodes[idx]).collect();

    Some((min_total_cost, path_nodes))
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);
    // println!("{:?}", Dot::new(&graph));

    if let Some((cost, path)) = shortest_path_all_nodes(&graph) {
        println!(
            "Shortest path visiting all nodes: {} [total cost: {cost}]",
            path.join(" -> ")
        );

        return Some(cost as u64);
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let input = indoc! {"London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141"};
        let result = part_one(input);
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
