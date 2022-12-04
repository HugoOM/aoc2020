use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.lines().collect::<Vec<&str>>();

    // Z -> Y -> X
    let mut space = initialize(contents);

    for _ in 0..6 {
        space = process(&space);
    }

    println!("Final Active Tally: {}", count_active(&space));
}

fn count_active(
    space: &HashMap<isize, HashMap<isize, HashMap<isize, HashMap<isize, bool>>>>,
) -> u32 {
    let mut active_count = 0;

    for w in space.values() {
        for z in w.values() {
            for y in z.values() {
                for &x in y.values() {
                    if x {
                        active_count += 1;
                    }
                }
            }
        }
    }

    active_count
}

fn process(
    current_space: &HashMap<isize, HashMap<isize, HashMap<isize, HashMap<isize, bool>>>>,
) -> HashMap<isize, HashMap<isize, HashMap<isize, HashMap<isize, bool>>>> {
    let mut expanded_space = current_space.clone();

    // Produce all new cubes to be evaluated this pass.
    for (w_i, w) in current_space.iter() {
        for (z_i, z) in w.iter() {
            for (y_i, y) in z.iter() {
                for (x_i, _) in y.iter() {
                    for x_n in -1isize..=1 {
                        for y_n in -1isize..=1 {
                            for z_n in -1isize..=1 {
                                for w_n in -1isize..=1 {
                                    if w_n == 0 && x_n == 0 && y_n == 0 && z_n == 0 {
                                        continue;
                                    }

                                    let p = (w_i + w_n, z_i + z_n, y_i + y_n, x_i + x_n);

                                    if !expanded_space.contains_key(&p.0) {
                                        expanded_space.insert(
                                            p.0,
                                            HashMap::<isize, HashMap<isize, HashMap<isize, bool>>>::new(),
                                        );
                                    }

                                    if !expanded_space.get(&p.0).unwrap().contains_key(&p.1) {
                                        expanded_space.get_mut(&p.0).unwrap().insert(
                                            p.1,
                                            HashMap::<isize, HashMap<isize, bool>>::new(),
                                        );
                                    }

                                    if !expanded_space
                                        .get(&p.0)
                                        .unwrap()
                                        .get(&p.1)
                                        .unwrap()
                                        .contains_key(&p.2)
                                    {
                                        expanded_space
                                            .get_mut(&p.0)
                                            .unwrap()
                                            .get_mut(&p.1)
                                            .unwrap()
                                            .insert(p.2, HashMap::<isize, bool>::new());
                                    }

                                    if !expanded_space
                                        .get(&p.0)
                                        .unwrap()
                                        .get(&p.1)
                                        .unwrap()
                                        .get(&p.2)
                                        .unwrap()
                                        .contains_key(&p.3)
                                    {
                                        expanded_space
                                            .get_mut(&p.0)
                                            .unwrap()
                                            .get_mut(&p.1)
                                            .unwrap()
                                            .get_mut(&p.2)
                                            .unwrap()
                                            .insert(p.3, false);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Evaluate all cubes - including those created during this pass.
    let mut processed_space = expanded_space.clone();
    for (w_i, w) in expanded_space.iter() {
        for (z_i, z) in w.iter() {
            for (y_i, y) in z.iter() {
                for (x_i, x) in y.iter() {
                    let mut active_neighbors_count = 0u32;

                    for x_n in -1isize..=1 {
                        for y_n in -1isize..=1 {
                            for z_n in -1isize..=1 {
                                for w_n in -1isize..=1 {
                                    if w_n == 0 && x_n == 0 && y_n == 0 && z_n == 0 {
                                        continue;
                                    }

                                    let p = (w_i + w_n, z_i + z_n, y_i + y_n, x_i + x_n);

                                    if !expanded_space.contains_key(&p.0)
                                        || !expanded_space.get(&p.0).unwrap().contains_key(&p.1)
                                        || !expanded_space
                                            .get(&p.0)
                                            .unwrap()
                                            .get(&p.1)
                                            .unwrap()
                                            .contains_key(&p.2)
                                        || !expanded_space
                                            .get(&p.0)
                                            .unwrap()
                                            .get(&p.1)
                                            .unwrap()
                                            .get(&p.2)
                                            .unwrap()
                                            .contains_key(&p.3)
                                    {
                                        continue;
                                    }

                                    if *expanded_space
                                        .get(&p.0)
                                        .unwrap()
                                        .get(&p.1)
                                        .unwrap()
                                        .get(&p.2)
                                        .unwrap()
                                        .get(&p.3)
                                        .unwrap()
                                    {
                                        active_neighbors_count += 1;
                                    }
                                }
                            }
                        }
                    }

                    let is_current_cube_active = *x;
                    let processed_cube = processed_space
                        .get_mut(&w_i)
                        .unwrap()
                        .get_mut(&z_i)
                        .unwrap()
                        .get_mut(&y_i)
                        .unwrap()
                        .get_mut(&x_i)
                        .unwrap();

                    if is_current_cube_active {
                        if active_neighbors_count != 2 && active_neighbors_count != 3 {
                            *processed_cube = false;
                        }
                    } else if active_neighbors_count == 3 {
                        *processed_cube = true;
                    }
                }
            }
        }
    }

    processed_space
}

fn initialize(
    rows: Vec<&str>,
) -> HashMap<isize, HashMap<isize, HashMap<isize, HashMap<isize, bool>>>> {
    let mut space: HashMap<isize, HashMap<isize, HashMap<isize, HashMap<isize, bool>>>> =
        HashMap::new();

    let rows: Vec<Vec<char>> = rows
        .clone()
        .iter()
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    space.insert(
        0,
        HashMap::<isize, HashMap<isize, HashMap<isize, bool>>>::new(),
    );
    space
        .get_mut(&0)
        .unwrap()
        .insert(0, HashMap::<isize, HashMap<isize, bool>>::new());

    for y in 0..rows.len() {
        space
            .get_mut(&0)
            .unwrap()
            .get_mut(&0)
            .unwrap()
            .insert(y as isize - 1, HashMap::<isize, bool>::new());

        for x in 0..rows[0].len() {
            space
                .get_mut(&0)
                .unwrap()
                .get_mut(&0)
                .unwrap()
                .get_mut(&(y as isize - 1))
                .unwrap()
                .insert(x as isize - 1, rows[rows.len() - 1 - y][x] == '#');
        }
    }

    space
}
