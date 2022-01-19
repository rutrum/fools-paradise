enum MenuState {
    Main,
    Options,
}

enum MenuItem {
    Start,
    Options,
    Sound,
}

impl MenuState {
    fn list(&self) -> Vec<&'static str> {
        use MenuState::*;
        match self {
            Main => vec![
                "Start",
                "Options",
            ];
        }
    }
}
