use crate::cavernos;
use crate::geometry::Rect;

pub trait Element {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect;
}

pub struct Clear<T: Element> {
    pub child: T,
}
impl<T: Element> Element for Clear<T> {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        for col in available_space.left..=available_space.right {
            for row in available_space.top..=available_space.bottom {
                os.set_char(col as u8, row as u8, 0);
            }
        }

        self.child.render(os, available_space)
    }
}

pub struct Padding<T: Element> {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
    pub child: T,
}
impl<T: Element> Element for Padding<T> {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        let padded_space = available_space.pad(self.top, self.right, self.bottom, self.left);
        self.child.render(os, padded_space)
    }
}

pub struct Stack {
    pub children: Vec<Box<dyn Element>>,
}
impl Element for Stack {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        let mut remaining_space = available_space;
        for child in &self.children {
            remaining_space = child.render(os, remaining_space);
        }
        remaining_space
    }
}

pub struct Style<T: Element> {
    pub bg_color: Option<u8>,
    pub fg_color: Option<u8>,
    pub child: T,
}
impl<T: Element> Element for Style<T> {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        for col in available_space.left..=available_space.right {
            for row in available_space.top..=available_space.bottom {
                if let Some(color) = self.bg_color {
                    os.set_bg_color(col as u8, row as u8, color);
                }
                if let Some(color) = self.fg_color {
                    os.set_fg_color(col as u8, row as u8, color);
                }
            }
        }

        self.child.render(os, available_space)
    }
}

pub struct Selector {
    pub is_active: bool,
    pub text: Vec<TextSegment>,
}
impl Element for Selector {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        let text_area = Rect {
            top: available_space.top,
            left: available_space.left + 2,
            right: available_space.right - 2,
            bottom: available_space.top,
        };
        let text = Text::from_segments(self.text.clone(), TextAlign::Middle);
        os.set_char(available_space.left as u8, available_space.top as u8, 27);
        os.set_char(available_space.right as u8, available_space.top as u8, 26);

        let bg_color = if self.is_active { Some(1) } else { None };
        let fg_color = if self.is_active { Some(0) } else { Some(1) };

        let child = Style {
            bg_color,
            fg_color,
            child: text,
        };
        child.render(os, text_area);

        // Consume one row
        available_space.pad(1, 0, 0, 0)
    }
}

pub struct Border<T: Element> {
    el: T,
}
impl<T: Element> Border<T> {
    pub fn new(child: T) -> Self {
        Border { el: child }
    }
}
impl<T: Element> Element for Border<T> {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        let top = available_space.top as u8;
        let left = available_space.left as u8;
        let bottom = available_space.bottom as u8;
        let right = available_space.right as u8;

        for col in available_space.left..=available_space.right {
            let x = col as u8;
            os.set_char(x, top, 196);
            os.set_char(x, bottom, 196);
        }
        for row in available_space.top..=available_space.bottom {
            let y = row as u8;
            os.set_char(left, y, 179);
            os.set_char(right, y, 179);
        }
        // Set corners
        os.set_char(left, top, 218);
        os.set_char(right, top, 191);
        os.set_char(left, bottom, 192);
        os.set_char(right, bottom, 217);

        // Render child
        self.el.render(os, available_space.pad(1, 1, 1, 1))
    }
}

pub enum TextAlign {
    Start,
    Middle,
}

pub struct Text {
    chars: Vec<u8>,
    horizontal_align: TextAlign,
}

#[derive(Clone)]
pub enum TextSegment {
    Text(String),
    Char(u8),
}

impl Text {
    pub fn from_segments(segments: Vec<TextSegment>, horizontal_align: TextAlign) -> Text {
        let mut chars = Vec::new();
        for segment in segments {
            match segment {
                TextSegment::Char(ref c) => chars.push(*c),
                TextSegment::Text(s) => {
                    for c in s.chars() {
                        if c.is_ascii() {
                            chars.push(c as u8);
                        }
                    }
                }
            }
        }
        Text {
            chars,
            horizontal_align,
        }
    }
}

impl Element for Text {
    fn render(&self, os: &mut cavernos::CavernOS, available_space: Rect) -> Rect {
        // How many rows?

        let mut words = Vec::new();
        let mut current_word = Vec::new();

        for c in &self.chars {
            // Space
            if *c == 32 {
                words.push(current_word);
                current_word = Vec::new();
            } else {
                current_word.push(*c);
            }
        }
        words.push(current_word);

        let mut lines = Vec::new();
        let mut current_line = Vec::new();

        let mut line_length = 0;
        let mut word_count = 0;
        for w in words {
            if w.len() + line_length > (available_space.width() as usize) {
                // Adding this word would overflow, start a new line
                lines.push((line_length + (word_count - 1), current_line));
                current_line = Vec::new();
                line_length = 0;
                word_count = 0;
            }

            line_length += w.len(); // Extra char for the space between words
            current_line.push(w);
            word_count += 1;
        }
        lines.push((line_length + (word_count - 1), current_line));

        let mut col;
        let mut row = available_space.top as u8;
        for line in lines {
            let (line_length, words) = line;
            col = match self.horizontal_align {
                TextAlign::Start => available_space.left as u8,
                TextAlign::Middle => {
                    let empty_space = available_space.width() - (line_length as i32);
                    (available_space.left as u8) + (empty_space / 2) as u8
                }
            };

            for word in words {
                for c in word {
                    os.set_char(col, row, c);
                    col += 1;
                }
                col += 1;
            }
            row += 1;
            if row > available_space.bottom as u8 + 1 {
                break;
            }
        }
        //cavernos::log(format!("{:?}", words));
        //panic!("Debug");

        Rect {
            top: row as i32,
            bottom: available_space.bottom,
            left: available_space.left,
            right: available_space.right,
        }
    }
}
