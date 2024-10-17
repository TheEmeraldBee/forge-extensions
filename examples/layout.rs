use std::{error::Error, time::Duration};

use ascii_forge::prelude::*;
use forge_widgets::buf_set::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window = Window::init_inline(5)?;

    loop {
        let mut layouts = layout(
            window.size(),
            &[
                (SetConstraint::Percentage(50), &[SetConstraint::Remaining]),
                (
                    SetConstraint::Amount(3),
                    &[
                        SetConstraint::Percentage(25),
                        SetConstraint::Percentage(50),
                        SetConstraint::Remaining,
                    ],
                ),
                (SetConstraint::Remaining, &[SetConstraint::Remaining]),
            ],
        );

        window.buffer().render_layout(&mut layouts);

        if event!(window, Event::Key(k) => k.code == KeyCode::Char('q')) {
            break;
        }

        window.update(Duration::from_millis(500))?;
    }

    Ok(())
}
