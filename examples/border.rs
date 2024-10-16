use std::error::Error;
use std::time::Duration;

use forge_widgets::border::Border;

use ascii_forge::prelude::*;
use forge_widgets::utils::CenteredOver;

fn main() -> Result<(), Box<dyn Error>> {
    // Create Ascii Forge Window
    let mut window = Window::init()?;

    // Create Border
    let border = Border::box_style(vec2(50, 5)).with_style(ContentStyle::new().blue());

    loop {
        let mut center = window.size();
        center.x /= 2;
        center.y /= 2;

        let text = "Press `Q` to quit!".green();

        render!(
            window,
            border.centered_on(center) => [ border ],
            text.centered_on(center) => [ text ]
        );

        if event!(window, Event::Key(k) => k.code == KeyCode::Char('q')) {
            break;
        }

        window.update(Duration::from_millis(500))?;
    }

    Ok(())
}
