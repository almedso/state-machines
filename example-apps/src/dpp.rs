//! Dining philosopher's problem
//!
//! Implementation example for Quantum Leaps Rust Like
//!
use log::{debug, info};
use qlrl::{ProcessingResult, State, StateMachineContext};

//----------------------------------------------------------------------------
// Type definitions for events, states, and state machine private data

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhilosopherId {
    Plato,
    Sokrates,
    Aristoteles,
}

#[derive(Debug, Clone)]
pub enum DppEvent {
    RequestLeftFork(PhilosopherId),
    RequestRightFork(PhilosopherId),
    FinishEating(PhilosopherId),
    ReleaseLeftFork(PhilosopherId),
    ReleaseRightFork(PhilosopherId),
    GrantLeftFork(PhilosopherId),
    GrantRightFork(PhilosopherId),
}

#[derive(PartialEq, Debug)]
pub struct PhilosopherData {
    id: PhilosopherId,
}

impl PhilosopherData {
    pub fn new(id: PhilosopherId) -> Self {
        PhilosopherData { id }
    }
}

#[derive(Debug, PartialEq)]
pub enum PhilosopherState {
    Think,
    Hungry,
    Eat,
}

#[derive(PartialEq, Debug)]
pub struct TableData {
    forks_available: [bool; 3],
}

impl TableData {
    pub fn new() -> Self {
        TableData {
            forks_available: [true; 3],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TableState {
    Operational,
}

//----------------------------------------------------------------------------
// state specific state handler functions

fn philosopher_think_entry<'a>(
    data: &'a mut PhilosopherData,
    context: &mut (dyn StateMachineContext<DppEvent> + 'a),
) -> () {
    info!("Think: {:?}", data.id);
    debug!(
        "Publish {:?} after {} ms",
        DppEvent::RequestLeftFork(data.id),
        1000
    );
    context.publish_delayed_event(1000, DppEvent::RequestLeftFork(data.id));
}

fn philosopher_think_dispatch<'a>(
    data: &'a mut PhilosopherData,
    _context: &mut (dyn StateMachineContext<DppEvent> + 'a),
    event: DppEvent,
) -> ProcessingResult<PhilosopherState> {
    match event {
        DppEvent::GrantLeftFork(philosopher) => {
            if philosopher == data.id {
                ProcessingResult::Transition(PhilosopherState::Hungry)
            } else {
                ProcessingResult::Ignored
            }
        }
        _ => ProcessingResult::Ignored,
    }
}

fn philosopher_hungry_entry<'a>(
    data: &'a mut PhilosopherData,
    context: &mut (dyn StateMachineContext<DppEvent> + 'a),
) -> () {
    info!("Hungry: {:?}", data.id);
    debug!("Publish {:?}", DppEvent::RequestRightFork(data.id),);
    context.publish_event(DppEvent::RequestRightFork(data.id));
}

fn philosopher_hungry_dispatch<'a>(
    data: &'a mut PhilosopherData,
    _context: &mut (dyn StateMachineContext<DppEvent> + 'a),
    event: DppEvent,
) -> ProcessingResult<PhilosopherState> {
    match event {
        DppEvent::GrantRightFork(philosopher) => {
            if philosopher == data.id {
                ProcessingResult::Transition(PhilosopherState::Eat)
            } else {
                ProcessingResult::Ignored
            }
        }
        _ => ProcessingResult::Ignored,
    }
}

fn philosopher_eat_entry<'a>(
    data: &'a mut PhilosopherData,
    context: &mut (dyn StateMachineContext<DppEvent> + 'a),
) -> () {
    info!("Eat: {:?}", data.id);
    debug!(
        "Publish {:?} after 1000 ms",
        DppEvent::FinishEating(data.id),
    );
    context.publish_delayed_event(100, DppEvent::FinishEating(data.id));
}

fn philosopher_eat_exit<'a>(
    data: &'a mut PhilosopherData,
    context: &mut (dyn StateMachineContext<DppEvent> + 'a),
) -> () {
    debug!(
        "Publish {:?} and {:?} ",
        DppEvent::ReleaseRightFork(data.id),
        DppEvent::ReleaseLeftFork(data.id),
    );
    context.publish_event(DppEvent::ReleaseLeftFork(data.id));
    context.publish_event(DppEvent::ReleaseRightFork(data.id));
}

fn philosopher_eat_dispatch<'a>(
    data: &'a mut PhilosopherData,
    _context: &mut (dyn StateMachineContext<DppEvent> + 'a),
    event: DppEvent,
) -> ProcessingResult<PhilosopherState> {
    match event {
        DppEvent::FinishEating(philosopher) => {
            if philosopher == data.id {
                ProcessingResult::Transition(PhilosopherState::Think)
            } else {
                ProcessingResult::Ignored
            }
        }
        _ => ProcessingResult::Ignored,
    }
}

fn table_operational_dispatch<'a>(
    data: &'a mut TableData,
    context: &mut (dyn StateMachineContext<DppEvent> + 'a),
    event: DppEvent,
) -> ProcessingResult<TableState> {
    match event {
        DppEvent::RequestLeftFork(philosopher) => {
            context.publish_event(DppEvent::GrantLeftFork(philosopher));
            data.forks_available[0] = false;
            ProcessingResult::Handled
        }
        DppEvent::RequestRightFork(philosopher) => {
            context.publish_event(DppEvent::GrantRightFork(philosopher));
            ProcessingResult::Handled
        }
        _ => ProcessingResult::Ignored,
    }
}

//----------------------------------------------------------------------------
// default states

fn init<D, E, S: PartialEq>() -> Option<State<D, E, S>> {
    None
}

fn entry<'a, D, E>(_data: &'a mut D, _context: &mut (dyn StateMachineContext<E> + 'a)) -> () {}

fn exit<'a, D, E>(_data: &'a mut D, _context: &mut (dyn StateMachineContext<E> + 'a)) -> () {}

//----------------------------------------------------------------------------
// static data structures
//
// Note:
//    These data structures require careful reviews since the compiler is not
//    able to find any inconsistencies

pub const PHILOSOPHER_STATES: [State<PhilosopherData, DppEvent, PhilosopherState>; 3] = [
    State::<PhilosopherData, DppEvent, PhilosopherState> {
        state: PhilosopherState::Think,
        super_state: None,
        entry: philosopher_think_entry,
        exit,
        init,
        dispatch: philosopher_think_dispatch,
    },
    State::<PhilosopherData, DppEvent, PhilosopherState> {
        state: PhilosopherState::Hungry,
        super_state: None,
        entry: philosopher_hungry_entry,
        exit,
        init,
        dispatch: philosopher_hungry_dispatch,
    },
    State::<PhilosopherData, DppEvent, PhilosopherState> {
        state: PhilosopherState::Eat,
        super_state: None,
        entry: philosopher_eat_entry,
        exit: philosopher_eat_exit,
        init,
        dispatch: philosopher_eat_dispatch,
    },
];

pub const TABLE_STATES: [State<TableData, DppEvent, TableState>; 1] =
    [State::<TableData, DppEvent, TableState> {
        state: TableState::Operational,
        super_state: None,
        entry,
        exit,
        init,
        dispatch: table_operational_dispatch,
    }];
