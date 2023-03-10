use crate::bits::{get_bit, modify_bit};

const BUTTON_COUNT: usize = 8;

#[derive(Clone, Copy)]
enum Button {
    Start,
    Select,
    Down,
    Up,
    Left,
    Right,
    A,
    B,
}

enum SelectedButtons {
    ActionButtons,
    DirectionButtons,
    None,
}

const ACTION_BUTTONS: &[Button] = &[Button::A, Button::B, Button::Select, Button::Start];
const DIRECTION_BUTTONS: &[Button] = &[Button::Right, Button::Left, Button::Up, Button::Down];

pub struct Joypad {
    buttons: [bool; BUTTON_COUNT],
    selected: SelectedButtons,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            buttons: [false; BUTTON_COUNT],
            selected: SelectedButtons::None,
        }
    }

    pub fn get_byte(&self) -> u8 {
        let mut b;

        let selected_buttons = match self.selected {
            SelectedButtons::ActionButtons => {
                b = modify_bit(0, 5, true); // action buttons
                ACTION_BUTTONS
            }

            SelectedButtons::DirectionButtons => {
                b = modify_bit(0, 4, true); // direction buttons
                DIRECTION_BUTTONS
            }

            SelectedButtons::None => return 0,
        };

        for (bit, button) in selected_buttons.iter().enumerate() {
            b = modify_bit(b, bit as u8, self.get_button(*button));
        }

        b
    }

    pub fn set_byte(&mut self, b: u8) {
        self.selected = if get_bit(b, 5) {
            SelectedButtons::ActionButtons
        } else if get_bit(b, 4) {
            SelectedButtons::DirectionButtons
        } else {
            SelectedButtons::None
        };
    }

    fn get_button(&self, button: Button) -> bool {
        self.buttons[button as usize]
    }

    #[allow(dead_code)]
    fn set_button(&mut self, button: Button, value: bool) {
        self.buttons[button as usize] = value;
    }
}
