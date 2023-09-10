use bevy::prelude::*;

#[derive(Event)]
pub enum TokenEvent {
    Spawn,
}

#[derive(Component)]
pub struct Token {}

impl Token {
    fn new(commands: &mut Commands) -> Self {
        commands.spawn(SpriteBundle {
            ..Default::default()
        });
        Self {}
    }
}

pub fn on_token_event(mut commands: Commands, mut event_reader: EventReader<TokenEvent>) {
    for e in event_reader.iter() {
        match e {
            TokenEvent::Spawn => Token::new(&mut commands),
        };
    }
}
