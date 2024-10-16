use std::fmt::Display;

use ascii_forge::prelude::*;

use crate::utils::SizedElement;

pub struct Border<R: Render> {
    pub size: Vec2,

    pub top: R,
    pub bottom: R,
    pub left: R,
    pub right: R,

    pub top_left: R,
    pub top_right: R,
    pub bottom_left: R,
    pub bottom_right: R,
}

impl<G: Render> SizedElement for Border<G> {
    fn size(&self) -> Vec2 {
        self.size
    }
}

impl<D: Display + Render> Border<StyledContent<D>> {
    pub fn with_style(mut self, style: ContentStyle) -> Self {
        *self.top.style_mut() = style;
        *self.bottom.style_mut() = style;
        *self.left.style_mut() = style;
        *self.right.style_mut() = style;
        *self.top_left.style_mut() = style;
        *self.top_right.style_mut() = style;
        *self.bottom_left.style_mut() = style;
        *self.bottom_right.style_mut() = style;
        self
    }
}

impl Border<StyledContent<&str>> {
    pub fn box_style(size: impl Into<Vec2>) -> Self {
        Self {
            size: size.into(),

            top: "─".stylize(),
            bottom: "─".stylize(),
            left: "│".stylize(),
            right: "│".stylize(),

            top_left: "┌".stylize(),
            top_right: "┐".stylize(),
            bottom_left: "└".stylize(),
            bottom_right: "┘".stylize(),
        }
    }
}

impl<R: Render> Render for Border<R> {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        // Render Sides
        for x in (loc.x + 1)..(self.size.x + loc.x) {
            render!(
                buffer,
                vec2(x, loc.y) => [ self.bottom ],
                vec2(x, loc.y + self.size.y - 1) => [ self.top ]
            );
        }

        for y in (loc.y + 1)..(self.size.y - 1 + loc.y) {
            render!(
                buffer,
                vec2(loc.x, y) => [ self.left ],
                vec2(loc.x + self.size.x, y) => [ self.right ]
            );
        }

        // Render Corners
        render!(
            buffer,
            loc => [ self.top_left ],
            vec2(loc.x + self.size.x, loc.y) => [ self.top_right ],
            vec2(loc.x, loc.y + self.size.y - 1) => [ self.bottom_left ],
            vec2(loc.x + self.size.x, loc.y + self.size.y - 1) => [ self.bottom_right ],
        );

        loc.x += self.size.x;
        loc.y += self.size.y;
        loc
    }
}
