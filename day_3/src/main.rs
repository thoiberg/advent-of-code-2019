use std::collections::HashSet;
use std::fs::File;
use std::io::Error as ioError;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    // println!("input data: {:?}", read_and_process_input().unwrap()[0]);
    let moves = read_and_process_input().unwrap();
    let first_move_list = create_list_of_points(moves[0].clone());
    let second_move_list = create_list_of_points(moves[1].clone());

    // TODO What does <_> mean?
    let mut common_points: HashSet<_> = first_move_list.intersection(&second_move_list).collect();

    common_points.remove(&(0, 0));

    let mut distances: Vec<i32> = common_points
        .into_iter()
        .map(|point| find_manhattan_distance(point.clone()))
        .collect();

    distances.sort();

    println!("{}", distances[0]);
}

fn find_manhattan_distance(coordinate: (i32, i32)) -> i32 {
    let central_port = (0, 0);

    return ((coordinate.0 - central_port.0).abs()) + ((coordinate.1 - central_port.1).abs());
}

fn create_list_of_points(moves: String) -> HashSet<(i32, i32)> {
    let moves_vec: Vec<&str> = moves.split(",").collect();
    let mut coordinates: Vec<(i32, i32)> = vec![(0, 0)];

    for movement in moves_vec {
        let direction = movement.chars().nth(0).unwrap();
        let distance_to_move = &movement[1..].parse::<i32>().unwrap();
        match direction {
            'R' => {
                let mut step = coordinates.last().unwrap().0;
                for coordinate in (step + 1..step + distance_to_move + 1) {
                    coordinates.push((coordinate, coordinates.last().unwrap().1));
                }
            }
            'U' => {
                let mut step = coordinates.last().unwrap().1;
                for coordinate in (step + 1..step + distance_to_move + 1) {
                    coordinates.push((coordinates.last().unwrap().0, coordinate));
                }
            }
            'L' => {
                let mut counter = distance_to_move.to_owned();

                while counter > 0 {
                    coordinates.push((
                        (coordinates.last().unwrap().0) - 1,
                        coordinates.last().unwrap().1,
                    ));
                    counter -= 1;
                }
            }
            'D' => {
                let mut counter = distance_to_move.to_owned();

                while counter > 0 {
                    coordinates.push((
                        (coordinates.last().unwrap().0),
                        coordinates.last().unwrap().1 - 1,
                    ));
                    counter -= 1;
                }
            }
            _ => panic!("Unexpected direction, exiting"),
        }
    }

    // TODO figure out what cloned() does
    HashSet::from_iter(coordinates.iter().cloned())
}

fn read_and_process_input() -> Result<Vec<String>, ioError> {
    let mut raw_input_data = File::open("src/input_data")?;
    let mut contents = String::new();
    raw_input_data.read_to_string(&mut contents)?;
    Ok(contents
        .split("\n")
        .map(|route| String::from(route))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_can_find_the_manhattan_distance_for_closest_intersection() {
    //     let test_data = vec![
    //         String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
    //         String::from("U62,R66,U55,R34,D71,R55,D58,R83"),
    //     ];
    //     assert_eq!(find_manhattan_distance(test_data), 159);
    // }
    #[test]
    fn test_create_list_of_points_process_up() {
        assert_eq!(
            create_list_of_points(String::from("U4")),
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]
        );
    }

    #[test]
    fn test_create_list_of_points_process_right() {
        assert_eq!(
            create_list_of_points(String::from("R4")),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
    }

    #[test]
    fn test_create_list_of_points_process_left() {
        assert_eq!(
            create_list_of_points(String::from("L4")),
            vec![(0, 0), (-1, 0), (-2, 0), (-3, 0), (-4, 0)]
        )
    }

    #[test]
    fn test_create_list_of_points_process_down() {
        assert_eq!(
            create_list_of_points(String::from("D4")),
            vec![(0, 0), (0, -1), (0, -2), (0, -3), (0, -4)]
        )
    }

    #[test]
    fn test_create_list_of_points_handle_multiple_movements() {
        assert_eq!(
            create_list_of_points(String::from("U1,L3,R7,D2")),
            vec![
                (0, 0),
                (0, 1),
                (-1, 1),
                (-2, 1),
                (-3, 1),
                (-2, 1),
                (-1, 1),
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
                (4, 1),
                (4, 0),
                (4, -1),
            ]
        );
    }
}
