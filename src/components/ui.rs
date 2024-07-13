use bevy::prelude::{Component, Entity};

#[derive(Debug, Component, Copy, Clone)]
pub struct LogContainer;

#[derive(Debug, Component, Copy, Clone)]
pub struct LogText;

#[derive(Debug, Component, Copy, Clone)]
pub struct HpNode;

#[derive(Debug, Component, Copy, Clone)]
pub struct HpText;

/// Entity IDs of messagess that should be displayed by the combat log
#[derive(Debug, Component, Copy, Clone)]
pub struct Messages([Option<Entity>; 5]);

impl Messages {
    pub fn new(inner: [Option<Entity>; 5]) -> Self {
        Messages(inner)
    }

    /// We want to only print X messages, so when we push new one, to be displayed, the last one has to be removed
    /// Hence this returns the [Entity] of the message that should be despawned.
    pub fn add(&mut self, entity: Entity) -> Option<Entity> {
        let last = self.0.last().map(Clone::clone).flatten();

        self.0 = [Some(entity), self.0[0], self.0[1], self.0[2], self.0[3]];
        last
    }

    pub fn to_vec(&self) -> Vec<Entity> {
        self.0
            .iter()
            .filter_map(Clone::clone)
            .collect::<Vec<Entity>>()
    }
}
