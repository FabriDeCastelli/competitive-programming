use crate::bst::binary_search_trees::BST;

/// # Frogs and Mosquitoes
/// https://codeforces.com/problemset/problem/609/F
/// In this version we are not returning the number of mosquitoes eaten by each frog and its
/// tongue length, but the position of the frog and the size of the eaten mosquito.
///
/// # Arguments
///
/// * `frogs`: A vector of tuples with the position and the tongue length of each frog.
/// * `mosquitoes`: A vector of tuples with the position and the size of each mosquito.
///
/// returns: Vec<(i32, i32), Global>
///
/// \theta((n + m) \log (n + m))
pub fn frogs_and_mosquitoes(
    mut frogs: Vec<(i32, i32)>,
    mosquitoes: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let n = frogs.len();
    let mut m = mosquitoes.len();

    frogs = frogs.iter().map(|&(x, t)| (x, x + t)).collect();

    let mut frogs_bst = BST::from_vec(frogs.clone());

    frogs.sort();
    let mut mosquitoes_bst = BST::from_vec(mosquitoes.clone());
    let mut result = Vec::new();

    for mosquito in mosquitoes {
        for i in 1..n {
            let (x2, t2) = frogs[i];
            let (x1, t1) = frogs_bst.predecessor((x2, t2)).unwrap();

            if x1 <= x2 && t2 <= t1 {
                frogs_bst.delete((x2, t2));
            } else if x1 <= x2 {
                frogs_bst.update((x2, t2), (t1, t2));
            }
        }

        let winning_frog = frogs_bst.predecessor(mosquito);

        match winning_frog {
            None => {}
            Some(frog) => {
                result.push((frog.0, mosquito.1));
                frogs_bst.update(frog, (frog.0, frog.1 + mosquito.1));
                mosquitoes_bst.delete(mosquito);

                m = m - 1;
                if m == 0 {
                    break;
                }
                while let Some(next_mosquito) = mosquitoes_bst.successor(frog) {
                    if m == 0 {
                        break;
                    } else if next_mosquito.0.le(&frog.1) {
                        mosquitoes_bst.delete(next_mosquito);
                        m = m - 1;
                        frogs_bst.update(frog, (frog.0, frog.1 + next_mosquito.1));
                        result.push((frog.0, mosquito.1));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    result
}

#[test]
fn test_frogs_and_mosquitoes() {
    let frogs = vec![(1, 2), (3, 2), (5, 2)];
    let mosquitoes = vec![(2, 1), (4, 1), (6, 1)];
    let result = frogs_and_mosquitoes(frogs, mosquitoes);
    assert_eq!(result, vec![(1, 1), (1, 1), (5, 1)]);
}
