use nih_plug_iced::{container, Color};

const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x44 as f32 / 255.0,
    0x4B as f32 / 255.0,
);

pub enum Theme {
    DARK,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::DARK
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::DARK => dark::Container.into(),
        }
    }
}

mod dark {
    use nih_plug_iced::{container, Color};

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style<'a>(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::WHITE),
                background: Color::from_rgb8(249, 40, 20).into(),
                border_color: Color::from_rgb8(229, 20, 0),
                border_width: 5.0,
                border_radius: 8.0,
            }
        }
    }
}
