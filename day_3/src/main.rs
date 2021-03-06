use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::Error as ioError;
use std::iter::FromIterator;

fn main() {
    println!("Part One: {}", part_one_solution());
    println!("Part Two: {}", part_two_solution());
}

#[derive(Eq, Debug, Clone)]
struct GridPoint {
    x: i32,
    y: i32,
    total_distance: u64,
}

impl Hash for GridPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for GridPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn part_one_solution() -> i32 {
    let moves = read_and_process_input().unwrap();
    let first_move_list = create_list_of_points(moves[0].clone());
    let second_move_list = create_list_of_points(moves[1].clone());

    let common_points: HashSet<_> = first_move_list.intersection(&second_move_list).collect();

    let mut distances: Vec<i32> = common_points
        .into_iter()
        .map(|point| find_manhattan_distance(point.clone()))
        .collect();

    distances.sort();

    distances[0]
}

fn part_two_solution() -> u64 {
    let moves = read_and_process_input().unwrap();
    let first_move_list = create_list_of_points(moves[0].clone());
    let second_move_list = create_list_of_points(moves[1].clone());

    let common_points: HashSet<_> = first_move_list.intersection(&second_move_list).collect();

    let mut distances: Vec<u64> = common_points
        .into_iter()
        .map(|point| {
            find_smallest_path_to_intersection(
                &first_move_list.get(&point).unwrap(),
                &second_move_list.get(&point).unwrap(),
            )
        })
        .collect();

    distances.sort();

    distances[0]
}

fn find_manhattan_distance(coordinate: GridPoint) -> i32 {
    let central_port = GridPoint {
        x: 0,
        y: 0,
        total_distance: 0,
    };

    ((coordinate.x - central_port.x).abs()) + ((coordinate.y - central_port.y).abs())
}

fn find_smallest_path_to_intersection(first_point: &GridPoint, second_point: &GridPoint) -> u64 {
    first_point.total_distance + second_point.total_distance
}

fn create_list_of_points(moves: String) -> HashSet<GridPoint> {
    let moves_vec: Vec<&str> = moves.split(',').collect();
    let mut coordinates: Vec<GridPoint> = vec![];

    for movement in moves_vec {
        let direction = movement.chars().nth(0).unwrap();
        let distance_to_move = movement[1..].parse::<i32>().unwrap().to_owned();
        let movement_steps = match direction {
            'R' => (1, 0),
            'U' => (0, 1),
            'L' => (-1, 0),
            'D' => (0, -1),
            _ => panic!("Unexpected direction, exiting"),
        };

        for _ in 1..=distance_to_move {
            let last_coordinate = coordinates
                .last()
                .unwrap_or(&GridPoint {
                    x: 0,
                    y: 0,
                    total_distance: 0,
                })
                .clone();
            coordinates.push(GridPoint {
                x: last_coordinate.x + movement_steps.0,
                y: last_coordinate.y + movement_steps.1,
                total_distance: last_coordinate.total_distance + 1,
            })
        }
    }

    HashSet::from_iter(coordinates.iter().cloned())
}

fn read_and_process_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents.split('\n').map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(), 870);
    }

    #[test]
    fn test_part_two_solution() {
        assert_eq!(part_two_solution(), 13698);
    }

    #[test]
    fn test_create_list_of_points_process_up() {
        assert_eq!(
            create_list_of_points(String::from("U4")),
            HashSet::from_iter(vec![
                GridPoint {
                    x: 0,
                    y: 1,
                    total_distance: 1,
                },
                GridPoint {
                    x: 0,
                    y: 2,
                    total_distance: 2,
                },
                GridPoint {
                    x: 0,
                    y: 3,
                    total_distance: 3,
                },
                GridPoint {
                    x: 0,
                    y: 4,
                    total_distance: 4,
                },
            ])
        )
    }

    #[test]
    fn test_create_list_of_points_process_right() {
        assert_eq!(
            create_list_of_points(String::from("R4")),
            HashSet::from_iter(vec![
                GridPoint {
                    x: 1,
                    y: 0,
                    total_distance: 1
                },
                GridPoint {
                    x: 2,
                    y: 0,
                    total_distance: 2
                },
                GridPoint {
                    x: 3,
                    y: 0,
                    total_distance: 3
                },
                GridPoint {
                    x: 4,
                    y: 0,
                    total_distance: 4
                },
            ])
        );
    }

    #[test]
    fn test_create_list_of_points_process_left() {
        assert_eq!(
            create_list_of_points(String::from("L4")),
            HashSet::from_iter(vec![
                GridPoint {
                    x: -1,
                    y: 0,
                    total_distance: 1
                },
                GridPoint {
                    x: -2,
                    y: 0,
                    total_distance: 2
                },
                GridPoint {
                    x: -3,
                    y: 0,
                    total_distance: 3
                },
                GridPoint {
                    x: -4,
                    y: 0,
                    total_distance: 4
                },
            ])
        )
    }

    #[test]
    fn test_create_list_of_points_process_down() {
        assert_eq!(
            create_list_of_points(String::from("D4")),
            HashSet::from_iter(vec![
                GridPoint {
                    x: 0,
                    y: -1,
                    total_distance: 1
                },
                GridPoint {
                    x: 0,
                    y: -2,
                    total_distance: 2
                },
                GridPoint {
                    x: 0,
                    y: -3,
                    total_distance: 3
                },
                GridPoint {
                    x: 0,
                    y: -4,
                    total_distance: 4
                },
            ])
        )
    }

    #[test]
    fn test_create_list_of_points_handle_multiple_movements() {
        let actual = create_list_of_points(String::from("U1,L3,R7,D2"));
        let expected = HashSet::from_iter(vec![
            GridPoint {
                x: 0,
                y: 1,
                total_distance: 1,
            },
            GridPoint {
                x: -1,
                y: 1,
                total_distance: 2,
            },
            GridPoint {
                x: -2,
                y: 1,
                total_distance: 3,
            },
            GridPoint {
                x: -3,
                y: 1,
                total_distance: 4,
            },
            GridPoint {
                x: -2,
                y: 1,
                total_distance: 5,
            },
            GridPoint {
                x: -1,
                y: 1,
                total_distance: 6,
            },
            GridPoint {
                x: 0,
                y: 1,
                total_distance: 7,
            },
            GridPoint {
                x: 1,
                y: 1,
                total_distance: 8,
            },
            GridPoint {
                x: 2,
                y: 1,
                total_distance: 9,
            },
            GridPoint {
                x: 3,
                y: 1,
                total_distance: 10,
            },
            GridPoint {
                x: 4,
                y: 1,
                total_distance: 11,
            },
            GridPoint {
                x: 4,
                y: 0,
                total_distance: 12,
            },
            GridPoint {
                x: 4,
                y: -1,
                total_distance: 13,
            },
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_manhattan_distance() {
        assert_eq!(
            find_manhattan_distance(GridPoint {
                x: 10,
                y: 10,
                total_distance: 20
            }),
            20
        );
    }

    #[test]
    fn test_find_manhattan_distance_with_negative_coordinates() {
        assert_eq!(
            find_manhattan_distance(GridPoint {
                x: -10,
                y: -10,
                total_distance: 20
            }),
            20
        );
    }

    #[test]
    fn test_find_smallest_path_to_intersection() {
        assert_eq!(
            find_smallest_path_to_intersection(
                &GridPoint {
                    x: 10,
                    y: 10,
                    total_distance: 20
                },
                &GridPoint {
                    x: 5,
                    y: 3,
                    total_distance: 8
                }
            ),
            28
        );
    }
}
