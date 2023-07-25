#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    vec,
};

use test_puzzles::get_level_180;
mod test_puzzles;

fn main() {
    // let puzzle = WaterPuzzle {
    //     flasks: vec![
    //         Flask {
    //             contents: vec![
    //                 Layer {
    //                     size: 2,
    //                     content: Liquid::Blue,
    //                 },
    //                 Layer {
    //                     size: 2,
    //                     content: Liquid::Green,
    //                 },
    //             ],
    //         },
    //         Flask {
    //             contents: vec![
    //                 Layer {
    //                     size: 2,
    //                     content: Liquid::Green,
    //                 },
    //                 Layer {
    //                     size: 2,
    //                     content: Liquid::Blue,
    //                 },
    //             ],
    //         },
    //         Flask { contents: vec![] },
    //     ],
    // };
    // solve(puzzle);

    // println!("{:?}", puzzle);
    // println!("{:?}", find_available_moves(&puzzle));
    let level = get_level_180();
    assert!(level.is_valid());
    solve(level);
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct WaterPuzzle {
    pub flasks: Vec<Flask>,
}

impl WaterPuzzle {
    fn make_leagal_move(&mut self, r#move: &Move) {
        let (from_move_amt, from_liquid) = {
            let flask = &self.flasks[r#move.from];

            let top = flask
                .top_layer()
                .expect("Tried to move from flask with no content");

            (top.size, top.content)
        };

        let flask = &self.flasks[r#move.to];
        let space_left = flask.space_left();
        let top = flask.top_layer();

        match top {
            Some(layer) => {
                if layer.content != from_liquid {
                    panic!("Tried bad move");
                }
                if r#move.ammount_to_move != u32::min(from_move_amt, space_left) {
                    panic!("Tried to move less/more than possilbe");
                }
            }
            None => {}
        }

        self.make_move(r#move);
    }

    fn make_move(&mut self, r#move: &Move) {
        // println!("Making move:");
        // println!("{:?}, {},\n {:#?}", r#move, self.flasks.len(), self.flasks);
        let liquid = self.flasks[r#move.from]
            .top_layer()
            .expect(
                format!(
                    "Could not make move {:?}, from: {:?}",
                    r#move, self.flasks[r#move.from]
                )
                .as_str(),
            )
            .content;

        let to_flask = &mut self.flasks[r#move.to];

        to_flask.add_to_top(liquid, r#move.ammount_to_move);

        let from_flask = &mut self.flasks[r#move.from];
        from_flask.remove_from_top(r#move.ammount_to_move);

        if !self.is_valid() {
            println!("Err invalid puzzle: ");
            println!("{:#?}", self);
            // println!("{}", self.flasks.len());

            // println!("{:?}", r#move);
            panic!("Made a move that made the water puzzle invalid");
        }
    }

    fn is_solved(&self) -> bool {
        self.flasks.iter().all(|flask| match flask.top_layer() {
            Some(top_layer) => top_layer.size == 4,
            None => true,
        })
    }

    fn is_valid(&self) -> bool {
        let liquid_ammount =
            self.flasks
                .iter()
                .fold(HashMap::<Liquid, u32>::new(), |mut hm, flask| {
                    for layer in flask.contents.iter() {
                        let entry = hm.entry(layer.content);
                        entry.and_modify(|e| *e += layer.size).or_insert(layer.size);
                    }

                    hm
                });

        liquid_ammount.iter().all(|liquid| *liquid.1 == 4)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Flask {
    pub contents: Vec<Layer>,
}

impl Flask {
    fn is_empty(&self) -> bool {
        return self.contents.is_empty();
    }

    fn top_layer(&self) -> Option<&Layer> {
        self.contents.last()
    }

    fn top_layer_mut(&mut self) -> Option<&mut Layer> {
        self.contents.last_mut()
    }

    fn space_left(&self) -> u32 {
        4 - self.contents.iter().fold(0, |acc, layer| acc + layer.size)
    }

    fn remove_from_top(&mut self, ammount: u32) {
        let top_layer = self.top_layer_mut().unwrap();

        assert!(top_layer.size >= ammount);
        if ammount < top_layer.size {
            top_layer.size -= ammount;
        } else if ammount == top_layer.size {
            self.contents.pop();
        }
    }

    fn add_to_top(&mut self, content: Liquid, ammount: u32) {
        if let Some(top_layer) = self.top_layer_mut() {
            if top_layer.content == content {
                top_layer.size += ammount;
                return;
            }
        }

        self.contents.push(Layer {
            size: ammount,
            content,
        });
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Layer {
    size: u32,
    content: Liquid,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
enum Liquid {
    Gray,
    Brown,
    Yellow,
    Magenta,
    Green,
    LightGreen,
    DarkGreen,
    Purple,
    DarkBlue,
    Blue,
    Pink,
    Red,
    LightBlue,
}

// fn solve_inner(
//     water_puzzle: &mut WaterPuzzle,
//     made_moves: &mut Vec<Move>,
//     seen: &mut HashSet<WaterPuzzle>,
//     deapth: u32,
// ) -> Option<Vec<Move>> {
//     if deapth > 50 {
//         return None;
//     }

//     if water_puzzle.is_solved() {
//         return Some(made_moves.to_owned());
//     }

//     if seen.contains(water_puzzle) {
//         return None;
//     }

//     seen.insert(water_puzzle.clone());

//     let avail_moves = find_available_moves(water_puzzle);
//     // println!("Searching deapth: {}", deapth);
//     for r#move in avail_moves {
//         water_puzzle.make_leagal_move(&r#move);
//         made_moves.push(r#move.clone());

//         if let Some(solution) = solve_inner(water_puzzle, made_moves, seen, deapth + 1) {
//             return Some(solution);
//         }

//         water_puzzle.make_move(&r#move.inverse());
//         made_moves.pop();
//     }

//     None
// }

fn solve(waterpuzzle: WaterPuzzle) {
    // println!(
    //     "{:?}",
    //     solve_inner(&mut waterpuzzle, &mut vec![], &mut HashSet::new(), 0)
    // );
    // return;

    let avail_moves = find_available_moves(&waterpuzzle);
    let mut search_queue = vec![(waterpuzzle, avail_moves, vec![])];
    let mut seen: HashSet<WaterPuzzle> = HashSet::new();

    let mut deapth = 0;
    while !search_queue.is_empty() {
        println!(
            "Searching deapth: {}. Total seen positions: {}",
            deapth,
            seen.len()
        );
        let mut next_search_queue = vec![];

        for (mut puzzle, possible_moves, prev_moves) in search_queue.into_iter() {
            if seen.contains(&puzzle) {
                // println!("Skipping because seen");
                continue;
            }

            for r#move in possible_moves.into_iter() {
                // println!("Checking move: {:?}", r#move);
                if puzzle.flasks[r#move.from].is_empty() {
                    println!("Moves made: {:?}", prev_moves);
                    println!("puzzle: {:?}", puzzle);
                }
                puzzle.make_move(&r#move);

                let move_inverse = r#move.inverse();

                if puzzle.is_solved() {
                    let mut solution = prev_moves.clone();
                    solution.push(r#move);

                    println!("Found solution! {:?}", solution);
                    println!("{:?}", puzzle);
                    return;
                }

                let available_moves = find_available_moves(&puzzle);
                if available_moves.len() > 0 {
                    let mut made_moves = prev_moves.clone();

                    made_moves.push(r#move);

                    let puzzle = puzzle.clone();
                    // puzzle.flasks.sort_unstable();

                    // if !seen.contains(&puzzle) {
                    //     seen.insert(puzzle.clone());
                    next_search_queue.push((puzzle.clone(), available_moves, made_moves));
                    // }
                }

                puzzle.make_move(&move_inverse);
            }
            seen.insert(puzzle);
        }
        search_queue = next_search_queue;
        deapth += 1;
    }

    println!("Searched the entire problem space!");
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Move {
    from: usize,
    to: usize,
    ammount_to_move: u32,
}

impl Move {
    fn inverse(&self) -> Move {
        Move {
            from: self.to,
            to: self.from,
            ammount_to_move: self.ammount_to_move,
        }
    }
}

fn find_available_moves(waterpuzzle: &WaterPuzzle) -> Vec<Move> {
    let mut available_moves: Vec<Move> = vec![];
    for (from_flask_id, from_flask) in waterpuzzle.flasks.iter().enumerate() {
        for (to_flask_id, to_flask) in waterpuzzle.flasks.iter().enumerate() {
            let from_flask_top_layer = match from_flask.top_layer() {
                Some(layer) => layer,
                None => continue,
            };

            if from_flask_id == to_flask_id {
                continue;
            }

            if from_flask.is_empty() {
                continue;
            }

            if let Some(to_top_layer) = to_flask.top_layer() {
                if to_top_layer.content != from_flask_top_layer.content {
                    continue;
                }

                let movable_ammount = u32::min(from_flask_top_layer.size, to_flask.space_left());
                if movable_ammount <= 0 {
                    continue;
                }

                available_moves.push(Move {
                    from: from_flask_id,
                    to: to_flask_id,
                    ammount_to_move: movable_ammount,
                });
            } else {
                // to flask is empty, we can always move the top layer there
                available_moves.push(Move {
                    from: from_flask_id,
                    to: to_flask_id,
                    ammount_to_move: from_flask_top_layer.size,
                });
            }
        }
    }

    available_moves
}

#[cfg(test)]
mod tests {
    use crate::{Flask, Layer, Liquid, WaterPuzzle};

    #[test]
    fn check_water_puzzle_valid() {
        let water_puzzle = WaterPuzzle {
            flasks: vec![
                Flask {
                    contents: vec![
                        Layer {
                            size: 1,
                            content: Liquid::Red,
                        },
                        Layer {
                            size: 2,
                            content: Liquid::Brown,
                        },
                    ],
                },
                Flask {
                    contents: vec![
                        Layer {
                            size: 2,
                            content: Liquid::Brown,
                        },
                        Layer {
                            size: 1,
                            content: Liquid::Red,
                        },
                    ],
                },
                Flask {
                    contents: vec![Layer {
                        size: 2,
                        content: Liquid::Red,
                    }],
                },
                Flask { contents: vec![] },
            ],
        };

        assert!(water_puzzle.is_valid());

        let water_puzzle = WaterPuzzle {
            flasks: vec![
                Flask {
                    contents: vec![
                        Layer {
                            size: 1,
                            content: Liquid::Red,
                        },
                        Layer {
                            size: 1,
                            content: Liquid::Brown,
                        },
                    ],
                },
                Flask {
                    contents: vec![
                        Layer {
                            size: 2,
                            content: Liquid::Brown,
                        },
                        Layer {
                            size: 1,
                            content: Liquid::Red,
                        },
                    ],
                },
                Flask {
                    contents: vec![Layer {
                        size: 2,
                        content: Liquid::Red,
                    }],
                },
                Flask { contents: vec![] },
            ],
        };
        assert!(!water_puzzle.is_valid());
    }
}
