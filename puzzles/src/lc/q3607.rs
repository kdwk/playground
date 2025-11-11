// https://leetcode.com/problems/power-grid-maintenance/

use std::collections::{HashMap, HashSet};

use stdext::{prelude::Assertable, recipe::Log};

pub fn answer(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let connections = connections.into_iter().map(|pair| (pair[0], pair[1]));
    let mut conn_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (source, dest) in connections {
        conn_map
            .entry(source)
            .and_modify(|dests| dests.push(dest))
            .or_insert(vec![dest]);
        conn_map
            .entry(dest)
            .and_modify(|dests| dests.push(source))
            .or_insert(vec![source]);
    }
    let mut disabled: HashSet<i32> = HashSet::new();
    let mut operation_results = vec![];
    for query in queries {
        let requested = query[1];
        match query[0] {
            1 => operation_results.push(maintenance_check(
                requested,
                &mut HashSet::new(),
                &conn_map,
                &disabled,
            )),
            2 => _ = disabled.insert(requested),
            _ => unimplemented!(),
        }
    }
    operation_results
}

fn maintenance_check(
    requested: i32,
    explored: &mut HashSet<i32>,
    conn_map: &HashMap<i32, Vec<i32>>,
    disabled: &HashSet<i32>,
) -> i32 {
    explored.insert(requested);
    if disabled.contains(&requested) {
        let network = (&conn_map).get(&requested);
        if let Some(network) = network {
            for dest in network {
                if !explored.contains(dest) {
                    let result = maintenance_check(*dest, explored, conn_map, disabled);
                    if result != -1 {
                        return result;
                    }
                }
            }
        }
        -1
    } else {
        requested
    }
}

pub fn test() {
    // answer(
    //     5,
    //     adapt([[1, 2], [2, 3], [3, 4], [4, 5]]),
    //     adapt([[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]]),
    // )
    // .must_be(vec![3, 2, 3]);
    // answer(3, adapt([]), adapt([[1, 1], [2, 1], [1, 1]])).must_be(vec![1, -1]);
    // answer(
    //     3,
    //     adapt([[2, 3], [1, 2], [1, 3]]),
    //     adapt([[2, 3], [2, 2], [2, 2], [1, 2], [1, 3]]),
    // )
    // .must_be(vec![1, 1]);
    answer(
        3,
        adapt([[3, 2], [1, 3], [2, 1]]),
        adapt([[2, 2], [1, 2], [1, 2]]),
    )
    .must_be(vec![1, 1]);
}

fn adapt<const N: usize>(arr: [[i32; 2]; N]) -> Vec<Vec<i32>> {
    arr.into_iter().map(Vec::from).collect()
}
