use super::*;

struct Data;
struct Event;
#[derive(PartialEq)]
enum StateName {
    TopIdle,
    TopOperational,
    SecondBusy,
    SecondWaiting,
    ThirdBusyGetReady,
    ThirdBusyProcess,
}

fn init<D, E, S: PartialEq>() -> Option<State<D, E, S>> { None }
fn entry<'a, D, E>(_data: &'a mut D, _context: &mut (dyn StateMachineContext<E> + 'a)) -> () {}
fn exit<'a, D, E>(_data: &'a mut D, _context: &mut (dyn StateMachineContext<E> + 'a)) -> () {}
fn dispatch<'a, D, E, S: PartialEq>(
    _data: &'a mut D,
    _context: &mut (dyn StateMachineContext<E> + 'a),
    _event: E,
) -> ProcessingResult<S> {
    ProcessingResult::Ignored
}

const COMPLEX_STATE_MACHINE_DEFINITION : [State<Data, Event, StateName>; 6 ] = [
    State::<Data, Event, StateName> { state: StateName::TopIdle, super_state: None, init, entry, exit, dispatch},
    State::<Data, Event, StateName> { state: StateName::TopOperational, super_state: None, init, entry, exit, dispatch},
    State::<Data, Event, StateName> { state: StateName::SecondBusy, super_state: Some(StateName::TopOperational), init, entry, exit, dispatch},
    State::<Data, Event, StateName> { state: StateName::SecondWaiting, super_state: Some(StateName::TopOperational), init, entry, exit, dispatch},
    State::<Data, Event, StateName> { state: StateName::ThirdBusyGetReady, super_state: Some(StateName::SecondBusy), init, entry, exit, dispatch},
    State::<Data, Event, StateName> { state: StateName::ThirdBusyProcess, super_state: Some(StateName::SecondBusy), init, entry, exit, dispatch},
];


#[test]
fn find_state_index_ok() {
    assert_eq!(0_usize, find_state_index(&COMPLEX_STATE_MACHINE_DEFINITION, StateName::TopIdle).unwrap());
    assert_eq!(1_usize, find_state_index(&COMPLEX_STATE_MACHINE_DEFINITION, StateName::TopOperational).unwrap());
    assert_eq!(2_usize, find_state_index(&COMPLEX_STATE_MACHINE_DEFINITION, StateName::SecondBusy).unwrap());
    assert_eq!(3_usize, find_state_index(&COMPLEX_STATE_MACHINE_DEFINITION, StateName::SecondWaiting).unwrap());
    assert_eq!(4_usize, find_state_index(&COMPLEX_STATE_MACHINE_DEFINITION, StateName::ThirdBusyGetReady).unwrap());
    assert_eq!(5_usize, find_state_index(&COMPLEX_STATE_MACHINE_DEFINITION, StateName::ThirdBusyProcess).unwrap());
}

#[test]
#[should_panic]
fn find_state_index_fail() {
    let state_machine_definitions = [
        State::<Data, Event, StateName> { state: StateName::TopIdle, super_state: None, init, entry, exit, dispatch},
        State::<Data, Event, StateName> { state: StateName::TopOperational, super_state: None, init, entry, exit, dispatch},
    ];
    find_state_index(&state_machine_definitions, StateName::SecondBusy).expect("Not found panic");
}
