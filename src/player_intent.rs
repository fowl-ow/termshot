use bevy::prelude::*;

use crate::IntentSystems;

fn player_intent_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, system.in_set(IntentSystems));
}

fn system() {}

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        todo!();
    }
}
