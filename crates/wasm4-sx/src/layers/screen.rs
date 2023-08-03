pub struct Screen {
    pub width: u16,
    pub height: u16,
}

const SCREEN: Screen = Screen {
    width: 160,
    height: 160,
};

impl Screen {
    pub fn get() -> &'static Self {
        &SCREEN
    }
}
