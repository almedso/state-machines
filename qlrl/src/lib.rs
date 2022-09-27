//! # State machine quantum leaps like
//!
#![no_std]
use core::cmp::PartialEq;

pub enum ProcessingResult<S> {
    Handled,
    Ignored,
    Transition(S),
    Top,           // only needed for hierarchical state machines
    SuperState(S), // only needed for hierarchical state machines
}

pub type EntryFn<D, E> =
    for<'a> fn(data: &'a mut D, context: &mut (dyn StateMachineContext<E> + 'a)) -> ();

pub type ExitFn<D, E> =
    for<'a> fn(data: &'a mut D, context: &mut (dyn StateMachineContext<E> + 'a)) -> ();

pub type InitFn<D, E, S> = fn() -> Option<State<D, E, S>>;

pub type DispatchFn<D, E, S> = for<'a> fn(
    data: &'a mut D,
    context: &mut (dyn StateMachineContext<E> + 'a),
    event: E,
) -> ProcessingResult<S>;

/// States of a state machine are arranged as an (const) array of states
///
/// Associated types:
///
/// - D the private data type of the state machine
/// - E the event type the state machine can process
/// - S the state (enum) type enumerating all the states
///
/// Note:
///     each state must be described in exactly one element of state machine
///     describing array
pub struct State<D, E, S>
where
    E: 'static,
    S: PartialEq,
{
    pub state: S,
    pub super_state: Option<S>,
    pub entry: EntryFn<D, E>,
    pub exit: ExitFn<D, E>,
    pub init: InitFn<D, E, S>,
    pub dispatch: DispatchFn<D, E, S>,
}

/// Trait providing an execution context to a state machine
///
/// The context acts as an execution environment. where the state machine
/// can trigger allowed actions
///
/// The contexts is dependent upon the type of events the state machine can
/// process (i.e. receive and emit)
pub trait StateMachineContext<E> {
    /// Pulbish an event
    fn publish_event(&mut self, e: E);

    // Publish an event after a certain delay in microseconds
    fn publish_delayed_event(&mut self, delay_in_ms: u64, e: E);
}

/// Minimal functionality a state machine must support
///
/// Organizing this as a trait allows to instanciate simple and hierarchical state machines
/// using the same abstract interface
pub trait StateMachine<E: Send> {
    /// Bring the state machine into it's initial state
    fn start<'a>(&mut self, context: &mut (dyn StateMachineContext<E> + 'a));

    /// Let the state machine process an event
    fn dispatch<'a>(&mut self, context: &mut (dyn StateMachineContext<E> + 'a), event: E);
}

#[derive(Debug, Clone)]
pub struct Error;

pub fn find_state_index<D, E, S: PartialEq>(
    state_list: &[State<D, E, S>],
    state: S,
) -> Result<usize, Error> {
    for (index, value) in state_list.iter().enumerate() {
        if value.state == state {
            return Ok(index);
        }
    }
    Err(Error)
}

pub mod fsm;

#[cfg(test)]
mod tests;