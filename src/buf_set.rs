use ascii_forge::prelude::*;

#[derive(Clone, Copy)]
pub enum SetDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
pub enum SetConstraint {
    Percentage(u16),
    Amount(u16),
    Remaining,
}

/// Used to build a layout from the given constraints
/**
Example:
```rust, no-run
let window = Window::init()?;
let buffers = layout(window.size(), &[SetConstraint::Percentage(30), &[SetConstraint::Amount(5), SetConstraint::Remaining]]);
```
*/
pub fn layout(size: Vec2, sets: &[(SetConstraint, &[SetConstraint])]) -> Vec<Vec<(Vec2, Buffer)>> {
    let mut vertical = vec![];
    let mut horizontal = vec![];
    for (constraint, set) in sets {
        vertical.push(*constraint);
        horizontal.push(set);
    }

    let buffers = buffer_set(size, SetDirection::Vertical, &vertical);
    let mut sets = vec![];
    for i in 0..buffers.len() {
        let size = buffers[i].0;
        sets.push(buffer_set(size, SetDirection::Horizontal, horizontal[i]));
    }
    sets
}

pub fn buffer_set(
    size: Vec2,
    dir: SetDirection,
    constraints: &[SetConstraint],
) -> Vec<(Vec2, Buffer)> {
    let start = match dir {
        SetDirection::Vertical => size.y,
        SetDirection::Horizontal => size.x,
    };
    let mut remaining = start;

    let mut offset = 0;

    let mut buffers = vec![];

    for constraint in constraints {
        match constraint {
            SetConstraint::Percentage(p) => {
                if remaining == 0 {
                    panic!("No space available for percentages.")
                }

                let percentage = *p as f32 / 100.0;
                let start = start as f32;
                let val = ((start * percentage) as u16).min(remaining);

                println!("{val}");

                remaining -= val;

                offset += val;
            }
            SetConstraint::Amount(n) => {
                if remaining < *n {
                    panic!("Space available can't hold the desired space.");
                }
                remaining -= n;
                buffers.push((offset, *n));
                offset += n;
            }
            SetConstraint::Remaining => {
                buffers.push((offset, remaining));
                remaining = 0;
            }
        }
    }

    buffers
        .into_iter()
        .map(|(pos, val)| match dir {
            SetDirection::Vertical => (vec2(0, pos), Buffer::new((size.x, val))),
            SetDirection::Horizontal => (vec2(pos, 0), Buffer::new((val, size.y))),
        })
        .collect::<Vec<(Vec2, Buffer)>>()
}

pub trait RenderLayoutExtensions {
    fn render_set(&self, set: &mut [(Vec2, Buffer)]);
    fn render_layout(&self, layouts: &mut Vec<Vec<(Vec2, Buffer)>>);
}

impl RenderLayoutExtensions for Buffer {
    fn render_set(&self, set: &mut [(Vec2, Buffer)]) {
        for layout in set.iter_mut() {
            self.render(layout.0, &mut layout.1);
        }
    }

    fn render_layout(&self, layouts: &mut Vec<Vec<(Vec2, Buffer)>>) {
        for layout in layouts.iter_mut() {
            self.render_set(layout)
        }
    }
}
