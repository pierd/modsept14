use ambient_api::prelude::*;
use packages::this::messages::Input;
use packages::unit_schema::components::vertical_velocity;

#[main]
pub fn main() {
    Input::subscribe(move |ctx, msg| {
        let Some(player_id) = ctx.client_entity_id() else {
            return;
        };

        if msg.flying {
            entity::add_component(player_id, vertical_velocity(), 0.1);
        }
    });
}
