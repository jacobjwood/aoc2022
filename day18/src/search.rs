use std::collections::HashSet;

pub fn search_for_contact(
    empty_cube: &(i32, i32, i32),
    max_cube_bounds: &(i32, i32, i32),
    min_cube_bounds: &(i32, i32, i32),
    cubes: &HashSet<(i32, i32, i32)>,
    explored_set: &mut HashSet<(i32, i32, i32)>,
    exposed_sides: &mut usize,
) {
    let x_left = (empty_cube.0 - 1, empty_cube.1, empty_cube.2);
    let x_right = (empty_cube.0 + 1, empty_cube.1, empty_cube.2);
    let x_up = (empty_cube.0, empty_cube.1 + 1, empty_cube.2);
    let x_down = (empty_cube.0, empty_cube.1 - 1, empty_cube.2);
    let x_forward = (empty_cube.0, empty_cube.1, empty_cube.2 + 1);
    let x_back = (empty_cube.0, empty_cube.1, empty_cube.2 - 1);

    // already explored
    if explored_set.contains(empty_cube) {
        return;
    } else {
        // add to set of explored
        explored_set.insert(*empty_cube);
    }

    let neighbor_vec: Vec<(i32, i32, i32)> = vec![x_left, x_right, x_up, x_down, x_forward, x_back]
        .into_iter()
        .filter(|x| {
            x.0 >= min_cube_bounds.0
                && x.1 >= min_cube_bounds.1
                && x.2 >= min_cube_bounds.2
                && x.0 <= max_cube_bounds.0
                && x.1 <= max_cube_bounds.1
                && x.2 <= max_cube_bounds.2
                && !explored_set.contains(x)
        })
        .collect();

    for n in neighbor_vec {
        if cubes.contains(&n) {
            *exposed_sides += 1;
            continue;
        }

        search_for_contact(
            &n,
            max_cube_bounds,
            min_cube_bounds,
            cubes,
            explored_set,
            exposed_sides,
        );
    }

    return;
}
