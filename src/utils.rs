use std::fmt::Display;

use ascii_forge::prelude::*;

#[macro_export]
macro_rules! impl_size_crude {
    ($($ident:ty),*) => {
        $(impl SizedElement for $ident {
            fn size(&self) -> Vec2 {
                let mut buf = Buffer::new(vec2(80, 80));
                render!(buf, vec2(0, 0) => [ self ]);
                buf.shrink();
                buf.size()
            }

        })*
    }
}

impl_size_crude! { String, &str }

impl<R: Render + SizedElement + Display> SizedElement for StyledContent<R> {
    fn size(&self) -> Vec2 {
        self.content().size()
    }
}

pub trait SizedElement {
    fn size(&self) -> Vec2;
}

pub trait CenteredOver {
    fn centered_on(&self, point: Vec2) -> Vec2;
}

impl<T: SizedElement> CenteredOver for T {
    fn centered_on(&self, mut point: Vec2) -> Vec2 {
        let size = self.size();
        point.x -= size.x / 2;
        point.y -= size.y / 2;
        point
    }
}
