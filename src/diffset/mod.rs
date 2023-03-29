/// Convert a vector of i32 to a vector of bool.
fn i32vec_to_boolvec(v: Vec<i32>, n: i32) -> Vec<bool> {
    let m = n * (n - 1) + 1;
    let mut ret = vec![false; m as usize];
    v.iter().for_each(|x| {
        ret[*x as usize] = true;
    });
    ret
}

/// Normalize x modulo n*(n-1)+1.
fn normalize(x: i32, n: i32) -> i32 {
    let m = n * (n - 1) + 1;
    let x = x % m;
    if x < 0 {
        x + m
    } else {
        x
    }
}

/// Perform a depth-first-search for a (n^2-n+1, n, 1) difference set.
fn dfs(n: i32, ret: &mut Vec<i32>, diff: &mut Vec<bool>, i: i32, j: i32) -> bool {
    let m = n * (n - 1) + 1;

    if i == n {
        true
    } else {
        // Find the next element in the difference set
        for k in j..m {
            let mut d1_diffs = ret.iter().map(|x| normalize(k - *x, n));
            let d2_diffs = ret.iter().map(|x| normalize(*x - k, n)).collect::<Vec<_>>();

            let valid = d1_diffs.all(|x| !d2_diffs.contains(&x));
            if !valid {
                continue;
            }

            let valid = ret.iter().all(|x| {
                !diff[normalize(k - *x, n) as usize] && !diff[normalize(*x - k, n) as usize]
            });
            if valid {
                ret.iter().for_each(|x| {
                    diff[normalize(k - *x, n) as usize] = true;
                    diff[normalize(*x - k, n) as usize] = true;
                });
                ret.push(k);

                if dfs(n, ret, diff, i + 1, k + 1) {
                    return true;
                }

                ret.pop();
                ret.iter().for_each(|x| {
                    diff[normalize(k - *x, n) as usize] = false;
                    diff[normalize(*x - k, n) as usize] = false;
                });
            }
        }
        false
    }
}

/// Find a (n^2-n+1, n, 1) difference set.
pub fn find_diffset(n: i32) -> Option<Vec<i32>> {
    let m = n * (n - 1) + 1;
    let mut ret: Vec<i32> = vec![0];
    let mut diff = vec![false; m as usize];

    if dfs(n, &mut ret, &mut diff, 1, 1) {
        Some(ret)
    } else {
        None
    }
}

/// Find a (n^2-n+1, n, 1)-SBIBD and guarantee that the i-th block do not contain element i.
/// Based on difference sets.
pub fn find_sbibd(n: i32) -> Option<Vec<Vec<bool>>> {
    match n {
        n if n < 3 => None,
        n if n > 12 => None,
        n => {
            let diffset = find_diffset(n);
            if diffset.is_none() {
                return None;
            }

            let diffset = diffset.unwrap();
            let mut ret = vec![];
            for i in 0..(n * (n - 1) + 1) {
                ret.push(i32vec_to_boolvec(
                    diffset
                        .iter()
                        .map(|x| normalize(*x + i + 1, n))
                        .collect::<Vec<_>>(),
                    n,
                ))
            }
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization() {
        assert_eq!(normalize(0, 3), 0);
        assert_eq!(normalize(1, 3), 1);
        assert_eq!(normalize(2, 3), 2);
        assert_eq!(normalize(3, 3), 3);
        assert_eq!(normalize(-1, 10), 90);
        assert_eq!(normalize(-2, 10), 89);
        assert_eq!(normalize(-3, 10), 88);
    }

    #[test]
    fn existence() {
        assert!(find_diffset(1 + 2).is_some());
        assert!(find_diffset(1 + 3).is_some());
        assert!(find_diffset(1 + 4).is_some());
        assert!(find_diffset(1 + 5).is_some());
        assert!(find_diffset(1 + 7).is_some());
        assert!(find_diffset(1 + 8).is_some());
        assert!(find_diffset(1 + 9).is_some());
        assert!(find_diffset(1 + 11).is_some());
    }

    #[test]
    fn inexistence() {
        assert!(find_diffset(1 + 6).is_none());
    }
}
