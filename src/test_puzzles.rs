use crate::{Flask, Layer, Liquid, WaterPuzzle};

pub fn get_level_180() -> WaterPuzzle {
    let mut puzzle = WaterPuzzle {
        flasks: vec![
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::LightGreen,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Purple,
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
                        size: 1,
                        content: Liquid::LightBlue,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Green,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Red,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Gray,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::Yellow,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::LightBlue,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Pink,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::DarkGreen,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::Yellow,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Green,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Pink,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Yellow,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::Purple,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Pink,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Magenta,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::LightGreen,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::Magenta,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Purple,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::DarkGreen,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Purple,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::DarkGreen,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::LightBlue,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Pink,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::DarkBlue,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::DarkBlue,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::LightBlue,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Green,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::DarkBlue,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::Red,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Gray,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Brown,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::LightGreen,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::DarkGreen,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Magenta,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Gray,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Red,
                    },
                ],
            },
            Flask {
                contents: vec![
                    Layer {
                        size: 1,
                        content: Liquid::Gray,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Magenta,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::DarkBlue,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::LightGreen,
                    },
                ],
            },
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
                    Layer {
                        size: 1,
                        content: Liquid::Green,
                    },
                    Layer {
                        size: 1,
                        content: Liquid::Yellow,
                    },
                ],
            },
            Flask { contents: vec![] },
            Flask { contents: vec![] },
        ],
    };

    let iter = puzzle.flasks.iter_mut();
    iter.for_each(|flask| flask.contents.reverse());

    puzzle
}

// Liqucontent::LightGreen "lightgreen",
// 1 "purple",
// Liqucontent::Brown "brown",
// Liqucontent::LightBlue "lightblue",
// Liqucontent::Green "green",
// Liqucontent::Red "red",
// Liqucontent::Gray "gray",
// Liqucontent::Yellow "yellow",
// Liqucontent::Pink "pink",
// Liqucontent::DarkGreen "darkgreen",
// Liqucontent::Magenta "magenta",
// Liqucontent::DarkBlue darkblue
