use linear_algebra::matrix::*;

pub struct Terminal {
    character_weight_width: usize,
    character_weight_height: usize,
    spacing: Option<Spacing>,
}

impl Terminal {
    pub fn new(character_weight_width: usize, character_weight_height: usize) -> Self {
        Self {
            character_weight_width,
            character_weight_height,
            spacing: None,
        }
    }

    /// A terminal usually has some spacing between each character.
    pub fn with_spacing(mut self, horizontal: usize, vertical: usize) -> Self {
        self.spacing = Some(Spacing {
            horizontal,
            vertical,
        });
        self
    }

    /// Transform a flattened [Vec<char>] ('buffer') to fit the width:height-ratio of the terminal.
    ///
    /// What does this mean? The characters in a terminal are not as wide as they are high, and thus "drawing"
    /// in the terminal is not as easy as thinking of each character as individual pixels. This function takes
    /// a buffer (where everything IS evenly sized/spaced) and transforms it to something that can be more
    /// properly printed to a terminal.
    ///
    /// The "pixel-sized" elements in the 'buffer' will be seen as having the same size as the lower
    /// of the terminals width/height weights. Then, each pixel will be mapped to fit in the larger of the two.
    fn adjust_buffer<'a>(
        &self,
        mut buffer: &'a mut Vec<char>,
        width: usize,
    ) -> &'a mut Vec<char> {
        let mut total_weight_width = self.character_weight_width;
        let mut total_weight_height = self.character_weight_height;

        if let Some(spacing) = &self.spacing {
            total_weight_width += spacing.horizontal;
            total_weight_height += spacing.vertical;
            todo!("Maybe works, but not tested...");
        }

        let largest_weight = if total_weight_width > total_weight_height {
            total_weight_width
        } else {
            total_weight_height
        };

        // let index_column = index % self.width;
        // let index_row = index / self.width;
        // let index_row_max = self.buffer.len() / self.width;

        // if index_column == 0 {
        //     println!();
        // }

        // if index_column == self.width / 2 {
        //     print!("|");
        // } else if index_row_max / 2 == index_row {
        //     print!("-");
        // } else {
        //     print!("{c}");
        // }

        buffer
    }
}

struct Spacing {
    horizontal: usize,
    vertical: usize,
}

pub trait CanvasTrait {
    fn get_width() -> usize;

    fn get_height() -> usize;

    /// Iterate over the [Canvas].
    /// Row 0, Column 1,2,3...; Row 1, Column 1,2,3...
    fn column_iterator<'a>() ->  std::slice::Iter<'a, char>;

    /// Iterate over the [Canvas].
    /// Column 0, Row 1,2,3...; Column 1, Row 1,2,3...
    fn row_iterator<'a>() ->  std::slice::Iter<'a, char>;
}
