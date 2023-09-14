use ambient_api::{core::messages::Frame, prelude::*};
use packages::this::messages::Input;

#[main]
pub fn main() {
    Frame::subscribe(|frame| {
        let (delta, input) = input::get_delta();
        Input {
            flying: input.keys.contains(&KeyCode::E),
        }
        .send_server_unreliable();
    });
}
