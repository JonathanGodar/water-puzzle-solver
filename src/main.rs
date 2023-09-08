#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    vec,
};

use image::{Rgb, ImageBuffer, RgbImage, GenericImageView};
use imageproc::drawing::Canvas;
use show_image::Color;

mod test_puzzles;

fn is_background_color(color: Rgb<u8>) -> bool {
    for channel in color.0.iter() {
        if channel > &25 {
            return false;
        }
    }

    true
}

fn is_flask_outline_color(color: Rgb<u8>) -> bool {
    for channel in color.0.iter() {
        if channel < &195 || channel > &205 {
            return false;
        }
    }
    return true;
}
const GAME_AREA_TOP_LEFT: (u32, u32) = (10, 222);
const GAME_AREA_BOTTOM_RIGHT: (u32, u32) = (832, 1582);


fn find_flask_interior_start(image: &RgbImage, flask_middle: u32, flask_top: u32) -> Option<u32> {
    for y in flask_top..image.height() {
        if is_background_color(*image.get_pixel(flask_middle, y)) {
            return Some(y);
        }
    }

    None
}

enum FlaskContentStartErr {
    InvalidFlask,
    NoContents
}

fn find_flask_content_start(image: &RgbImage, flask_middle: u32, flask_interior_start: u32) -> Result<u32, FlaskContentStartErr> {
    for y in flask_interior_start..image.height() {
        let pixel = *image.get_pixel(flask_middle, y);
        if is_flask_outline_color(pixel) {
            return Err(FlaskContentStartErr::NoContents)
        }
      
        if !is_background_color(pixel) {
            return Ok(y);
        }
    }

    Err(FlaskContentStartErr::InvalidFlask)
}

fn square_color_distance(color_1: Rgb<u8>, color_2: Rgb<u8>) -> u32 {
    (color_1[0] as u32).abs_diff(color_2[0] as u32).pow(2) + 
    (color_1[1] as u32).abs_diff(color_2[1] as u32).pow(2) + 
    (color_1[2] as u32).abs_diff(color_2[2] as u32).pow(2) 
}

fn get_flask_contents(image: &RgbImage, left_bound: u32, right_bound: u32, flask_top: u32) -> Vec<(u32, Rgb<u8>)>  {
    let flask_middle = (right_bound + left_bound) / 2;

    let flask_interior_start = find_flask_interior_start(image, flask_middle, flask_top).expect("Invalid flask");
    let flask_content_start_result = find_flask_content_start(image, flask_middle, flask_interior_start);

    let flask_content_start = match flask_content_start_result {
        Ok(flask_content_start) => flask_content_start,
        Err(FlaskContentStartErr::InvalidFlask) => panic!("Invalid flask"),
        Err(FlaskContentStartErr::NoContents) => return vec![],
    };


    let mut contents = vec![];
    struct ColorLayer {
        top_position: u32,
        bottom_position: u32,
        color: Rgb<u8>,
    }

    let mut prev_color_top = flask_content_start;
    let mut prev_color = *image.get_pixel(flask_middle, flask_content_start);

    let mut maybe_flask_bottom: Option<u32> = None;
    for y in flask_content_start..image.height()  {
        let curr_color = *image.get_pixel(flask_middle, y);
        
        if is_flask_outline_color(curr_color) {
            // Deals with antialiasing
            if (y-1) - prev_color_top > 5 {
                contents.push(ColorLayer {
                    top_position: prev_color_top,
                    bottom_position: y - 1,
                    color: prev_color,
                });
            }

            maybe_flask_bottom = Some(y);
            break;
        }

        let square_color_distance = square_color_distance(prev_color, curr_color);
        if square_color_distance > 20u32.pow(2) {
            println!("Adding color from: {}, to: {}", prev_color_top, y-1);
            contents.push(ColorLayer {
                top_position: prev_color_top,
                bottom_position: y - 1,
                color: prev_color,
            });

            prev_color = curr_color;
            prev_color_top = y;
        }
    }
    
    let flask_interior_end = maybe_flask_bottom.unwrap();
    
    let flask_inner_height = flask_interior_end - flask_interior_start; 
    let possible_liquid_height =  flask_inner_height as f32 * 0.9;
    
    contents.into_iter().map(|layer| {
        let a = (layer.bottom_position - layer.top_position) as f32 / possible_liquid_height * 4f32;
        (a.round() as u32, layer.color)
    }).collect()
}

fn find_flask_rows(image: &RgbImage) -> Vec<(u32, u32)> {
    let mut flask_rows = vec![];
    for y in GAME_AREA_TOP_LEFT.1..GAME_AREA_BOTTOM_RIGHT.1 {
        // TODO CHANGE TO HALF SCREEN
        for x in GAME_AREA_TOP_LEFT.0..(GAME_AREA_TOP_LEFT.0 + 120u32) {
            if is_flask_outline_color(*image.get_pixel(x, y)) && is_background_color(*image.get_pixel(x, y -1)) {
                flask_rows.push((x,y));
                break;
            }
        }
    }

    flask_rows
}

fn find_next_flask_left_bound(image: &RgbImage, flask_left_bound: u32, y: u32) -> Option<u32> {
    for x in flask_left_bound..image.width() {
        if is_flask_outline_color(*image.get_pixel(x, y)) {
            return Some(x);
        }
    }

    None
}

fn find_flask_right_bound(image: &RgbImage, x_start: u32, y: u32) -> Option<u32> {
    for x in x_start..image.width() {
        if is_background_color(*image.get_pixel(x, y)) {
            return Some(x - 1);
        }
    }

    None
}

fn find_flasks_in_row(image: &RgbImage, row: (u32, u32)) -> Vec<((u32, u32), u32)> {
    let mut last_searched_x = row.0;
    let y = row.1;

    let mut current_flask_left_bound = Some(row.0);

    // left and right bound
    let mut flask_top_bounds = vec![];


    loop {
        match current_flask_left_bound {
            Some(left_bound) => {
                let maybe_right_bound = find_flask_right_bound(image, last_searched_x + 1, y);

                match maybe_right_bound {
                    Some(right_bound) => {
                        flask_top_bounds.push(((left_bound, right_bound), row.1));

                        last_searched_x = right_bound;
                        current_flask_left_bound = None;
                    },
                    None => panic!("Unfinised flask"),
                }
            },
            None => {
                let maybe_new_flask_left_bound = find_next_flask_left_bound(image, last_searched_x, y);
                match maybe_new_flask_left_bound {
                    Some(left_bound) => {
                        last_searched_x = left_bound;
                        current_flask_left_bound = Some(left_bound);
                    },
                    None => break,
                }
            }
        }
    }

    return flask_top_bounds;
}

fn read_image(image: &RgbImage) -> (Vec<Flask>, Vec<u32>, HashMap<u32, Rgb<u8>) {
    let rows = find_flask_rows(&image);
    let flasks: Vec<_> = rows.into_iter().map(|row| find_flasks_in_row(image, row)).collect();
    
    let flask_count_in_rows: Vec<_> = flasks.iter().map(|row| row.len() as u32).collect();
    
    let flasks: Vec<_> = flasks.into_iter().flatten().map(|flask_pos| {
        get_flask_contents(image, flask_pos.0.0, flask_pos.0.1, flask_pos.1)
    }).collect();
    
    let mut colors = HashMap::new();
    let mut color_id_counter = 0;

    flasks.iter().map(|flask|{
        flask.iter().map(|(size, color)| {
            
        })
    });

    for flask in flasks {
        for color in flask.iter_mut() {
            colors.iter().find(|color_mapping| {
                if square_color_distance(color_mapping.0, color.1) < 10.pow(2) {
                    
                }
            })
        }
    }
    
    todo!()
}


#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_data = image::open("./test/test_level_81.png")?;
    let mut raw_image = image_data.to_rgb8();

    let row = find_flask_rows(&raw_image)[1];
    
    let first_flask = find_flasks_in_row(&raw_image, row)[2];
    
    let flask_contents = get_flask_contents(&raw_image, first_flask.0.0, first_flask.0.1, first_flask.1);
    println!("flask_contents: {:?}", flask_contents);

    show_image_until_escape(raw_image);

    Ok(())

    // println!("{:?}", );

    // for row in find_flask_rows(&raw_image)  {
    //     let flasks = find_flasks_in_row(&raw_image, row.clone());
    //     for flask in flasks {
    //         imageproc::drawing::draw_line_segment_mut(
    //             &mut raw_image,
    //             (flask.0 as f32, row.1 as f32),
    //             (flask.1 as f32, row.1 as f32),
    //             Rgb([255, 127, 80])
    //         );
    //     }

    // }

    // raw_image.enumerate_pixels_mut().for_each(|pix| {
    //     if is_flask_outline_color(*pix.2) {
    //         *pix.2 = Rgb([255, 127, 80]);
    //     }
    // });




    // let puzzle = WaterPuzzle {
    // flasks: vec![
    //     Flask {
    //         contents: vec![
    //             Layer {
    //                 size: 2,
    //                 content: Liquid::Blue,
    //             },
    //             Layer {
    //                 size: 2,
    //                 content: Liquid::Green,
    //             },
    //         ],
    //         id: 1,
    //     },
    //     Flask {
    //         contents: vec![
    //             Layer {
    //                 size: 2,
    //                 content: Liquid::Green,
    //             },
    //             Layer {
    //                 size: 2,
    //                 content: Liquid::Blue,
    //             },
    //         ],
    //         id: 2,
    //     },
    //     Flask {
    //         contents: vec![],
    //         id: 3,
    //     },
    // ],
    // };
    // solve(puzzle);
    // 1 -> 3
    // 2 -> 1
    // 3 -> 2

    // let level = get_level_81();
    // assert!(level.is_valid());
    // solve(level);
}

fn show_image_until_escape(image: impl Into<show_image::Image>) {
    let window = show_image::create_window("image", Default::default()).unwrap();
    window.set_image("image-001", image).unwrap();
    
    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
    for event in window.event_channel().unwrap() {
      if let show_image::event::WindowEvent::CloseRequested(_evt) = event {
        break;
      } else if let show_image::event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(show_image::event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
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

    fn position_move_to_id_move(&self, r#move: &Move) -> Move {
        Move {
            from: self.flasks[r#move.from].id,
            to: self.flasks[r#move.to].id,
            ammount_to_move: r#move.ammount_to_move,
        }
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
                .fold(HashMap::<LiquidId, u32>::new(), |mut hm, flask| {
                    for layer in flask.contents.iter() {
                        let entry = hm.entry(layer.content);
                        entry.and_modify(|e| *e += layer.size).or_insert(layer.size);
                    }

                    hm
                });
        liquid_ammount.iter().all(|liquid| *liquid.1 == 4)
    }
}

#[derive(Clone, Debug)]
pub struct Flask {
    pub contents: Vec<Layer>,
    id: usize,
}

impl PartialEq for Flask {
    fn eq(&self, other: &Self) -> bool {
        self.contents.eq(&other.contents)
    }
}

impl Eq for Flask {}

impl Hash for Flask {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.contents.hash(state)
    }
}

impl PartialOrd for Flask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Flask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.contents.cmp(&other.contents)
    }
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

    fn add_to_top(&mut self, content: LiquidId, ammount: u32) {
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
    content: LiquidId,
}

// #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
type LiquidId = u32;
// enum Liquid {
//     Gray,
//     Brown,
//     Yellow,
//     Magenta,
//     Green,
//     LightGreen,
//     DarkGreen,
//     Purple,
//     DarkBlue,
//     Blue,
//     Pink,
//     Red,
//     LightBlue,
// }

fn solve_dfs(
    water_puzzle: &mut WaterPuzzle,
    made_moves: &mut Vec<Move>,
    seen: &mut HashSet<WaterPuzzle>,
    max_search_deapth: u32,
) -> Option<Vec<Move>> {
    if max_search_deapth <= 0 {
        return None;
    }

    if water_puzzle.is_solved() {
        return Some(made_moves.to_owned());
    }

    if seen.contains(water_puzzle) {
        return None;
    }
    seen.insert(water_puzzle.clone());

    let avail_moves = find_available_moves(water_puzzle);
    for r#move in avail_moves {
        water_puzzle.make_leagal_move(&r#move);
        made_moves.push(r#move.clone());

        if let Some(solution) = solve_dfs(water_puzzle, made_moves, seen, max_search_deapth - 1) {
            return Some(solution);
        }

        water_puzzle.make_move(&r#move.inverse());
        made_moves.pop();
    }

    None
}

fn solve(waterpuzzle: WaterPuzzle) {
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
            for r#move in possible_moves.into_iter() {
                puzzle.make_move(&r#move);

                let move_inverse = r#move.inverse();

                if puzzle.is_solved() {
                    let mut solution = prev_moves.clone();
                    solution.push(puzzle.position_move_to_id_move(&r#move));

                    println!("Found solution! {:#?}", solution);
                    println!("{:?}", puzzle);
                    return;
                }

                let mut sorted_puzzle = puzzle.clone();
                sorted_puzzle.flasks.sort_unstable();
                if !seen.contains(&sorted_puzzle) {
                    let available_moves = find_available_moves(&sorted_puzzle);
                    if available_moves.len() > 0 {
                        let mut made_moves = prev_moves.clone();

                        made_moves.push(puzzle.position_move_to_id_move(&r#move));
                        next_search_queue.push((
                            sorted_puzzle.clone(),
                            available_moves,
                            made_moves,
                        ));
                    }
                    seen.insert(sorted_puzzle);
                }

                puzzle.make_move(&move_inverse);
            }
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
    use image::Rgb;

    use crate::{Flask, Layer, LiquidId, WaterPuzzle, square_color_distance};
    
    #[test]
    fn test_square_color_distance() {
        let square_distance = square_color_distance(
            Rgb([1, 1, 1]),
            Rgb([10, 10, 10]),
        );
        
        let expected = 9u32.pow(2) * 3;
        
        assert_eq!(square_distance, expected);
        
    }
    // #[test]
    // fn check_water_puzzle_valid() {
    //     let water_puzzle = WaterPuzzle {
    //         flasks: vec![
    //             Flask {
    //                 contents: vec![
    //                     Layer {
    //                         size: 1,
    //                         content: LiquidId::Red,
    //                     },
    //                     Layer {
    //                         size: 2,
    //                         content: LiquidId::Brown,
    //                     },
    //                 ],
    //                 id: 0,
    //             },
    //             Flask {
    //                 contents: vec![
    //                     Layer {
    //                         size: 2,
    //                         content: LiquidId::Brown,
    //                     },
    //                     Layer {
    //                         size: 1,
    //                         content: LiquidId::Red,
    //                     },
    //                 ],
    //                 id: 1,
    //             },
    //             Flask {
    //                 contents: vec![Layer {
    //                     size: 2,
    //                     content: Liquid::Red,
    //                 }],
    //                 id: 2,
    //             },
    //             Flask {
    //                 contents: vec![],
    //                 id: 3,
    //             },
    //         ],
    //     };

    //     assert!(water_puzzle.is_valid());

    //     let water_puzzle = WaterPuzzle {
    //         flasks: vec![
    //             Flask {
    //                 contents: vec![
    //                     Layer {
    //                         size: 1,
    //                         content: LiquidId::Red,
    //                     },
    //                     Layer {
    //                         size: 1,
    //                         content: LiquidId::Brown,
    //                     },
    //                 ],
    //                 id: 0,
    //             },
    //             Flask {
    //                 contents: vec![
    //                     Layer {
    //                         size: 2,
    //                         content: Liquid::Brown,
    //                     },
    //                     Layer {
    //                         size: 1,
    //                         content: Liquid::Red,
    //                     },
    //                 ],
    //                 id: 1,
    //             },
    //             Flask {
    //                 contents: vec![Layer {
    //                     size: 2,
    //                     content: Liquid::Red,
    //                 }],
    //                 id: 2,
    //             },
    //             Flask {
    //                 contents: vec![],
    //                 id: 3,
    //             },
    //         ],
    //     };
    //     assert!(!water_puzzle.is_valid());
    // }
}
