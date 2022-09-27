//! Dining Philosophers Problem

use example_apps::dpp::{DppEvent, PhilosopherData, PhilosopherId, TableData, PHILOSOPHER_STATES, TABLE_STATES};
use log::{self, info};
use qlrl::{fsm::FiniteStateMachine, StateMachine};

use threads_on_host::ThreadedContext;

fn main() {
    env_logger::init();
    info!("Start state machine runtime context using threads, channels and busses");

    let mut context = ThreadedContext::<DppEvent>::new();
    context.add(Box::new(FiniteStateMachine::new(
        &PHILOSOPHER_STATES,
        PhilosopherData::new(PhilosopherId::Aristoteles),
    )) as Box<dyn StateMachine<DppEvent> + Send + 'static>);

    context.add(Box::new(FiniteStateMachine::new(
        &PHILOSOPHER_STATES,
        PhilosopherData::new(PhilosopherId::Plato),
    )) as Box<dyn StateMachine<DppEvent> + Send + 'static>);

    context.add(Box::new(FiniteStateMachine::new(
        &PHILOSOPHER_STATES,
        PhilosopherData::new(PhilosopherId::Sokrates),
    )) as Box<dyn StateMachine<DppEvent> + Send + 'static>);

    context.add(
        Box::new(FiniteStateMachine::new(&TABLE_STATES, TableData::new()))
            as Box<dyn StateMachine<DppEvent> + Send + 'static>,
    );

    context.run();
}
