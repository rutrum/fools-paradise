use crate::wasm4::sys::*;

/// Returns the current mouse position
pub fn mouse_position() -> (i16, i16) {
    return unsafe {(
        *MOUSE_X.as_ref().unwrap(), 
        *MOUSE_Y.as_ref().unwrap()
    )}
}

/// The list of inputs for the first gamepad, and the mouse
#[derive(Clone, Copy, Debug)]
pub enum Button {
    Primary,
    Secondary,
    Left,
    Right,
    Up,
    Down,
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

impl Button {
    /// The cooresponding bytes for each button
    fn byte(self) -> u8 {
        use Button::*;
        match self {
            Primary => BUTTON_1,
            Secondary => BUTTON_2,
            Left => BUTTON_LEFT,
            Right => BUTTON_RIGHT,
            Up => BUTTON_UP,
            Down => BUTTON_DOWN,
            MouseLeft => MOUSE_LEFT,
            MouseRight => MOUSE_RIGHT,
            MouseMiddle => MOUSE_MIDDLE,
        }
    }

    /// Returns if the button is a mouse button
    pub fn is_mouse(self) -> bool {
        !self.is_gamepad()
    }

    /// Returns if the button is part of the first gamepad
    pub fn is_gamepad(self) -> bool {
        use Button::*;
        !matches!(self, MouseLeft | MouseRight | MouseMiddle)
    }
}

/// Stores the current and previous gamepad and mouse state
#[derive(Default, Debug)]
pub struct Controls {
    gamepad: u8,
    pressed: u8,
    held: u8,

    mouse_gamepad: u8,
    mouse_pressed: u8,
    mouse_held: u8,
}

impl Controls {
    /// A default, null `Controls`
    pub fn new() -> Self {
        Self {
            gamepad: 0,
            pressed: 0,
            held: 0,

            mouse_gamepad: 0,
            mouse_pressed: 0,
            mouse_held: 0,
        }
    }

    /// Reads the current gamepad and mouse buttons state
    pub fn next(&mut self) {
        let current = unsafe { *GAMEPAD1 };
        let mouse_current = unsafe { *MOUSE_BUTTONS };

        let next = Self {
            gamepad: current,
            pressed: current & (current ^ self.gamepad),
            held: self.gamepad & current,

            mouse_gamepad: mouse_current,
            mouse_pressed: mouse_current & (mouse_current ^ self.mouse_gamepad),
            mouse_held: self.mouse_gamepad & mouse_current,
        };
        *self = next;
    }

    /// Returns if a button is pressed this frame, but was not pressed last frame
    pub fn pressed(&self, but: Button) -> bool {
        if but.is_gamepad() {
            self.pressed & but.byte() > 0
        } else {
            self.mouse_pressed & but.byte() > 0
        }
    }

    /// Returns if a button is pressed this frame and was pressed last frame as well
    pub fn held(&self, but: Button) -> bool {
        if but.is_gamepad() {
            self.held & but.byte() > 0
        } else {
            self.mouse_held & but.byte() > 0
        }
    }

    pub fn pressed_or_held(&self, but: Button) -> bool {
        self.held(but) || self.pressed(but)
    }
}
