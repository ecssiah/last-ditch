use crate::simulation::{
    admin::{Admin, Mode},
    population::Population,
    time::Time,
    world::World,
    SETTLEMENT_PERIOD,
};

pub struct State {
    pub admin: Admin,
    pub time: Time,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new() -> State {
        let state = State {
            admin: Admin::new(),
            time: Time::new(),
            world: World::new(),
            population: Population::new(),
        };

        state
    }

    pub fn calculate_work(&mut self) {
        self.time.calculate_work();
    }

    pub fn has_work(&self) -> bool {
        self.time.has_work()
    }

    pub fn generate(&mut self) {
        self.world.generate();
        self.population.generate();
    }

    pub fn settle(&mut self) {
        self.admin.settlement_tick += 1;

        self.tick();

        if self.admin.settlement_tick > SETTLEMENT_PERIOD {
            self.admin.mode = Mode::Simulate;
        }
    }

    pub fn tick(&mut self) {
        let tick = &self.time.tick;

        self.world.tick(tick);
        self.population.tick(tick);

        self.time.tick();
    }
}
