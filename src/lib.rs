mod diffset;
pub use diffset::{find_diffset, find_sbibd as find_sbibd_ds};

/// Generate all r-combinations of a given set of n i32 elements.
/// The result is a vector of vectors of i32.
///
/// Generated by ChatGPT
pub fn generate_rcombs(elems: &Vec<i32>, r: i32) -> Vec<Vec<i32>> {
    assert!(elems.len() >= r as usize);

    let n = elems.len();
    let mut comb = vec![0; r as usize];
    let mut result = Vec::new();

    fn generate_combinations(
        elems: &Vec<i32>,
        n: usize,
        r: i32,
        comb: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        pos: usize,
        start: usize,
    ) {
        if pos == r as usize {
            result.push(comb.clone());
            return;
        }

        for i in start..n {
            comb[pos] = elems[i];
            generate_combinations(elems, n, r, comb, result, pos + 1, i + 1);
        }
    }

    generate_combinations(elems, n, r, &mut comb, &mut result, 0, 0);
    result
}

/// Check whether an object whose primary is node can recover given the
/// erased nodes list.
///
/// # Arguments
///
/// * `primary`: the primary node of the object
/// * `n`: the number of data chunks to encode together
/// * `p`: max fault tolerance
pub fn solve_erasure(
    primary: i32,
    n: i32,
    p: i32,
    affinity: &Vec<Vec<bool>>,
    erased: &Vec<i32>,
) -> i32 {
    solve_erasure_search(
        primary,
        n,
        p,
        affinity,
        erased,
        vec![false; (n * (n - 1) + 1) as usize],
        1,
    )
}

const INFTY: i32 = 99999999;

fn solve_erasure_search(
    primary: i32,
    n: i32,
    p: i32,
    affinity: &Vec<Vec<bool>>,
    erased: &Vec<i32>,
    met: Vec<bool>,
    depth: i32,
) -> i32 {
    // let indent: String = " ".to_string().repeat(7);

    assert!(erased.len() <= p as usize);
    assert!(erased.iter().all(|x| *x >= 0 && *x < (n * (n - 1) + 1)));

    // println!(
    //     "{}SEARCHING depth: {}, n: {}, primary: {}, erased: {:?}",
    //     indent.repeat((depth - 1) as usize),
    //     depth,
    //     n,
    //     primary,
    //     erased
    // );

    let mut met = met.clone();
    met[primary as usize] = true;

    // Check if the object itself is alive
    if !erased.contains(&primary) {
        // println!(
        //     "{} - [WARNING] object itself is alive, this shouldn't happen!",
        //     indent.repeat((depth - 1) as usize)
        // );
        return depth;
    }
    // println!(
    //     "{} - primary is erased",
    //     indent.repeat((depth - 1) as usize)
    // );

    // Find which nodes can hold a parity of the object
    assert_eq!(affinity.len(), (n * (n - 1) + 1) as usize);
    let parity_can_reside = affinity
        .iter()
        .enumerate()
        .filter(|(_, row)| row[primary as usize])
        .map(|(i, _)| i as i32)
        .collect::<Vec<_>>();
    assert_eq!(parity_can_reside.len(), n as usize);
    // println!(
    //     "{} - parity can reside in {:?}",
    //     indent.repeat((depth - 1) as usize),
    //     parity_can_reside
    // );

    // Enumerate all possibilities of parity placements
    use std::cmp::{max, min};
    let parity_placements = generate_rcombs(&parity_can_reside, p + 1);

    parity_placements
        .iter()
        .map(|placement| {
            // println!(
            //     "{} - checking placement {:?}",
            //     indent.repeat((depth - 1) as usize),
            //     placement
            // );

            // Check if under the given placement, there is a chance for direct recovery
            let direct = placement.iter().any(|parity| {
                if erased.contains(parity) {
                    return false;
                }
                erased
                    .iter()
                    .all(|i| *i == primary || !affinity[*parity as usize][*i as usize])
            });
            if direct {
                // println!(
                //     "{} --- direct recovery possible",
                //     indent.repeat((depth - 1) as usize)
                // );
                return depth;
            } else {
                // println!(
                //     "{} --- direct recovery impossible",
                //     indent.repeat((depth - 1) as usize),
                // );
            }

            // If there is no such chance, recursively check if the object can be recovered
            // by recovering other objects first
            // println!(" --- no chance for direct recovery, checking recursively");
            let parity_alive = placement
                .iter()
                .filter(|parity| !erased.contains(parity))
                .collect::<Vec<_>>();
            // println!(
            //     "{} --- parities alive: {:?}",
            //     indent.repeat((depth - 1) as usize),
            //     parity_alive
            // );

            let ret = parity_alive
                .iter()
                .map(|parity| {
                    // Get the members of the parity
                    let members = affinity[**parity as usize]
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| **x)
                        .map(|(i, _)| i as i32)
                        .collect::<Vec<_>>();
                    // println!(
                    //     "{} --- members of parity {}: {:?}",
                    //     indent.repeat((depth - 1) as usize),
                    //     parity,
                    //     members
                    // );

                    // This parity cannot consist of objects we have already met in the search, except the primary
                    // Return INFTY to indicate that choosing this parity will result in infinite search depth
                    if members.iter().any(|i| *i != primary && met[*i as usize]) {
                        return INFTY;
                    }

                    // Find the maximum search depth among all dependencies
                    members
                        .iter()
                        .map(|i| {
                            // Return -INFTY to prevent it from affecting the results
                            if met[*i as usize] {
                                return -INFTY;
                            }
                            match erased.contains(i) {
                                true => {
                                    // println!(
                                    //     "{}   --- into recovery of {} ...",
                                    //     indent.repeat((depth - 1) as usize),
                                    //     i
                                    // );
                                    let ret = solve_erasure_search(
                                        *i,
                                        n,
                                        p,
                                        affinity,
                                        erased,
                                        met.clone(),
                                        depth + 1,
                                    );
                                    // match ret {
                                    //     d if d > -INFTY && d < INFTY => println!(
                                    //         "{}   --- recovery branch {} successful with depth {}",
                                    //         indent.repeat((depth - 1) as usize),
                                    //         i,
                                    //         d,
                                    //     ),
                                    //     _ => println!(
                                    //         "{}   --- \u{1b}[31mrecovery branch {} failed\u{1b}[39m",
                                    //         indent.repeat((depth - 1) as usize),
                                    //         i
                                    //     ),
                                    // };
                                    ret
                                }
                                false => depth,
                            }
                        })
                        .reduce(|x, y| max(x, y))
                        .unwrap()
                })
                .reduce(|x, y| min(x, y)) // Return the minimum depth of the search using the best parity choice
                .unwrap();
            // if ret == INFTY {
            //     println!(
            //         "{} --- [\u{1b}[31mERROR\u{1b}[39m] no valid solution for placement {:?}!",
            //         indent.repeat((depth - 1) as usize),
            //         *placement
            //     );
            // }
            ret
        })
        .reduce(|x, y| max(x, y)) // Return the worst case depth of the search
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination() {
        assert_eq!(
            generate_rcombs(&vec![0, 1, 2], 2),
            vec![vec![0, 1], vec![0, 2], vec![1, 2]]
        );
        assert_eq!(
            generate_rcombs(&vec![0, 1, 2, 3], 2),
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3]
            ]
        );
    }

    #[test]
    fn ec_4_works() {
        let (n, m, p) = (4, 13, 3);
        let nodes = (0..m).collect::<Vec<_>>();
        let affinity = find_sbibd_ds(n).unwrap();
        let erased = generate_rcombs(&nodes, p);
        erased
            .iter()
            .map(|case| {
                if !case.contains(&0) {
                    return 1;
                }
                let d = solve_erasure(0, n, p, &affinity, case);
                assert!(d != INFTY);
                d
            })
            .reduce(|x, y| std::cmp::max(x, y)) // Return the worst case depth of search
            .unwrap();
    }

    #[test]
    fn ec_works() {
        for n in 3..=9 {
            let m = n * (n - 1) + 1;
            let nodes = (0..m).collect::<Vec<_>>();

            let affinity = find_sbibd_ds(n);
            if affinity.is_none() {
                continue;
            }

            let affinity = affinity.unwrap();
            assert!(affinity.iter().enumerate().all(|(i, row)| !row[i]));
            for i in 0..m {
                assert!(!affinity[i as usize][i as usize]);
            }

            for p in 1..=std::cmp::min(n - 1, 6) {
                let erased = generate_rcombs(&nodes, p);
                let depth = erased
                    .iter()
                    .map(|case| {
                        if !case.contains(&0) {
                            return 1;
                        }
                        let d = solve_erasure(0, n, p, &affinity, case);
                        assert!(d != INFTY);
                        d
                    })
                    .reduce(|x, y| std::cmp::max(x, y)) // Return the worst case depth of search
                    .unwrap();
                println!("n = {}, p = {}, depth = {}", n, p, depth);
            }
            println!();
        }
    }
}
