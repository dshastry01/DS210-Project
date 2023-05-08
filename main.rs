use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// function to read the data file
fn read_file(path: &str) -> Result<Vec<(u64, u64)>, std::io::Error> {
    let mut edges = vec![];
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let node1 = parts[0].parse::<u64>().unwrap();
            let node2 = parts[1].parse::<u64>().unwrap();
            edges.push((node1, node2));
        }
    }
    Ok(edges)
}

// function to clean the data
fn clean_data(edges: &[(u64, u64)]) -> (HashMap<u64, HashSet<u64>>, usize) {
    let mut adj_list: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut max_vertex_id = 0;
    for &(node1, node2) in edges {
        max_vertex_id = max_vertex_id.max(node1).max(node2);
        adj_list.entry(node1).or_default().insert(node2);
        adj_list.entry(node2).or_default().insert(node1);
    }
    (adj_list, max_vertex_id as usize + 1)
}

// function to find the degree distribution
fn degree_distribution(adj_list: &HashMap<u64, HashSet<u64>>) -> HashMap<usize, usize> {
    let mut dist = HashMap::new();
    for (_, neighbors) in adj_list {
        let degree = neighbors.len();
        *dist.entry(degree).or_default() += 1;
    }
    dist
}

// function to find the average degree
fn average_degree(adj_list: &HashMap<u64, HashSet<u64>>) -> f64 {
    let degree_sum: usize = adj_list.values().map(|neighbors| neighbors.len()).sum();
    let num_vertices = adj_list.len();
    degree_sum as f64 / num_vertices as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_degree() {
        let edges = vec![(1, 2), (1, 3), (2, 3), (3, 4)];
        let (adj_list, _) = clean_data(&edges);
        assert_eq!(average_degree(&adj_list), 2.0);
    }
}

fn main() {
    let edges = read_file("twitter_combined.txt").unwrap();
    let (adj_list, num_vertices) = clean_data(&edges);
    println!("Number of vertices: {}", num_vertices);
    println!("Number of edges: {}", edges.len());

    // sorts the degree in descending order
    let degree_dist = degree_distribution(&adj_list);
    let mut degree_vec: Vec<(usize, usize)> = degree_dist.into_iter().collect();
    degree_vec.sort_by_key(|x| std::cmp::Reverse(x.0)); 

    println!("Degree distribution:");
    for (degree, count) in degree_vec {
        println!("{}: {}", degree, count);
    }
    println!("Average degree: {}", average_degree(&adj_list));
}
