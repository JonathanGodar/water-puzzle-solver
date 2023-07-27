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
                id: 0,
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
                id: 1,
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
                id: 2,
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
                id: 3,
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
                id: 4,
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
                id: 5,
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
                id: 6,
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
                id: 7,
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
                id: 8,
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
                id: 9,
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
                id: 10,
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
                id: 11,
            },
            Flask {
                contents: vec![],
                id: 12,
            },
            Flask {
                contents: vec![],
                id: 13,
            },
        ],
    };

    let iter = puzzle.flasks.iter_mut();
    iter.for_each(|flask| flask.contents.reverse());

    puzzle
}

pub fn get_level_81() -> WaterPuzzle {
    WaterPuzzle {
        flasks: vec![
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::LightGreen,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Gray,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Purple,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Yellow,
                        size: 1,
                    },
                ],
                id: 1,
            },
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::Gray,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::DarkGreen,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Yellow,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Brown,
                        size: 1,
                    },
                ],
                id: 2,
            },
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::DarkGreen,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Brown,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Gray,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Brown,
                        size: 1,
                    },
                ],
                id: 3,
            },
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::Purple,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::LightGreen,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::DarkBlue,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Yellow,
                        size: 1,
                    },
                ],
                id: 4,
            },
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::Purple,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::DarkBlue,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Purple,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::DarkGreen,
                        size: 1,
                    },
                ],
                id: 5,
            },
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::DarkBlue,
                        size: 2,
                    },
                    Layer {
                        content: Liquid::LightGreen,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::DarkGreen,
                        size: 1,
                    },
                ],
                id: 6,
            },
            Flask {
                contents: vec![
                    Layer {
                        content: Liquid::Brown,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Gray,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::Yellow,
                        size: 1,
                    },
                    Layer {
                        content: Liquid::LightGreen,
                        size: 1,
                    },
                ],
                id: 7,
            },
            Flask {
                contents: vec![],
                id: 8,
            },
            Flask {
                contents: vec![],
                id: 9,
            },
        ],
    }
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
