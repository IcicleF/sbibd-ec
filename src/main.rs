fn is_valid(m: i32, inc: &Vec<Vec<bool>>) -> bool {
    assert_eq!(inc.len(), m as usize);
    for i in 0..m {
        for j in (i + 1)..m {
            let common = inc[i as usize]
                .iter()
                .zip(inc[j as usize].iter())
                .filter(|(x, y)| **x && **y)
                .count();
            if common != 1 {
                return false;
            }
        }
    }
    true
}

// fn dfs(i: i32, n: i32, inc: &Vec<Vec<bool>>) -> Option<Vec<Vec<bool>>> {
//     let m = n * (n - 1) + 1;
//     if i == m {
//         return match is_valid(m, inc) {
//             true => Some(inc.clone()),
//             false => None,
//         };
//     }

//     use sbibd::generate_rcombs;
//     let pos = (0..m).collect::<Vec<_>>();
//     let placements = generate_rcombs(&pos, n);
//     for cand in placements {
//         let valid = cand
//             .iter()
//             .all(|x| *x != i && !inc[i as usize][*x as usize] && !inc[*x as usize][i as usize]);
//         if !valid {
//             continue;
//         }

//         let mut new_inc = inc.clone();
//         for j in 0..n {
//             new_inc[i as usize][cand[j as usize] as usize] = true;
//         }
//         let ret = dfs(i + 1, n, &new_inc);
//         if ret.is_some() {
//             return ret;
//         }
//     }
//     None
// }

fn main() {
    let n: i32 = 4;
    let m = (n * (n - 1) + 1) as usize;

    let mut inc = vec![vec![false; m]; m];
    let base: [usize; 4] = [1, 2, 4, 10];
    for i in 0..m {
        for j in base {
            inc[i][(i + j) % m] = true;
        }
    }
    assert!(is_valid(m as i32, &inc));
    for i in 0..m {
        for j in 0..m {
            assert!(!inc[i][j] || !inc[j][i]);
        }
    }
}
