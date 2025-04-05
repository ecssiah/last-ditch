//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod actions;
pub mod block;
pub mod chunk;
pub mod consts;
pub mod physics;
pub mod population;
pub mod state;
pub mod structure;
pub mod time;
pub mod views;
pub mod world;

pub use block::Block;
pub use chunk::Chunk;
pub use consts::*;

use crate::simulation::{actions::Actions, views::Views};
use actions::Action;
use physics::Physics;
use state::State;
use std::{
    sync::{Arc, RwLock},
    thread,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    actions: Actions,
    state: State,
    physics: Physics,
    views: Arc<RwLock<Views>>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let actions = Actions::new(action_rx);
        let state = State::new();
        let physics = Physics::new();

        let views = Arc::new(RwLock::new(Views::new()));

        let simulation = Self {
            actions,
            state,
            physics,
            views,
        };

        log::info!("Simulation Initialized");

        simulation
    }

    pub fn run(&mut self) {
        self.state.generate();
        self.physics.generate(&self.state);

        self.generate_views();

        loop {
            self.update();
        }
    }

    pub fn get_views(&self) -> Arc<RwLock<Views>> {
        Arc::clone(&self.views)
    }

    fn generate_views(&mut self) {
        let mut views = self.views.write().unwrap();

        views.generate(&self.state);
    }

    fn update(&mut self) {
        self.state.calculate_work();

        while self.state.has_work() {
            self.actions.tick(&mut self.state);
            self.state.tick();
            self.physics.tick(&mut self.state);

            self.tick_views();
        }

        thread::sleep(SIMULATION_WAIT_DURATION);
    }

    fn tick_views(&mut self) {
        let mut views = self.views.write().unwrap();

        views.tick(&self.state);
    }
}
