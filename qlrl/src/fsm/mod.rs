//! Finite State Machine processor
//!
//!
use core::cmp::PartialEq;

use super::{find_state_index, ProcessingResult, State, StateMachine, StateMachineContext};

pub struct FiniteStateMachine<D: 'static, E: 'static, S: PartialEq + 'static> {
    index: usize,
    state_list: &'static [State<D, E, S>],
    data: D,
}

impl<D, E, S: PartialEq> FiniteStateMachine<D, E, S> {
    pub fn new(state_list: &'static [State<D, E, S>], data: D) -> Self {
        FiniteStateMachine {
            state_list,
            index: 0,
            data, // data is moved
        }
    }
}

impl<D, E, S> StateMachine<E> for FiniteStateMachine<D, E, S>
where
    S: PartialEq,
    E: Send,
{
    /// Dispatch an event
    fn dispatch<'a>(&mut self, context: &mut (dyn StateMachineContext<E> + 'a), event: E) {
        match (self.state_list[self.index].dispatch)(&mut self.data, context, event) {
            ProcessingResult::Ignored | ProcessingResult::Handled => (),
            ProcessingResult::Transition(new_state) => {
                (self.state_list[self.index].exit)(&mut self.data, context);
                self.index = find_state_index(self.state_list, new_state).expect("State specification not found ");
                (self.state_list[self.index].entry)(&mut self.data, context);
            }
            ProcessingResult::SuperState(_current_state) => (), // relevant only for hierarchical state machines
            ProcessingResult::Top => (), // relevant only for hierarchical state machines
        }
    }

    /// Start the state machine i.e. let the state machine perform its
    /// initial transition from start to it's first state
    ///
    /// ```mermaid
    /// [*] --> FirstState
    /// ```
    fn start<'a>(&mut self, context: &mut (dyn StateMachineContext<E> + 'a)) {
        (self.state_list[self.index].entry)(&mut self.data, context);
    }
}

#[cfg(test)]
mod tests;
