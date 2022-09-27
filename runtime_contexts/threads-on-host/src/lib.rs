//! State machine execution context
//!
//! - uses threads and mpsc to inject events
//! - each state machine runs in a dedicated thread
//!
use bus::Bus;
use log::{debug};
use std::{
    sync::mpsc,
    thread::{sleep, spawn, JoinHandle},
    time,
    fmt::Debug,
    marker::{Send, Sync},
};
use ctrlc;

use qlrl::{StateMachine, StateMachineContext};


#[derive(Clone, Debug)]
pub enum ContextEvent<E: Clone + Debug + Send + Sync> {
    Start,
    Stop,
    Envelope(E),
}

pub struct WorkerContext<E: Clone + Debug + Send + Sync>(mpsc::SyncSender<ContextEvent<E>>);

impl <E: Clone + Debug + Send + Sync> StateMachineContext<E> for WorkerContext<E> {
    fn publish_event(&mut self, e: E) {
        self.0.send(ContextEvent::Envelope(e)).unwrap();  // panic in case of an error
    }

    fn publish_delayed_event(&mut self, delay_in_ms: u64, e: E) {
        let millis = time::Duration::from_millis(delay_in_ms);
        sleep(millis);
        self.0.send(ContextEvent::Envelope(e)).unwrap();  // panic in case of an error
    }
}

pub fn sm_worker<E: Clone + Debug + Sync + Send >(
    sm: Box<dyn StateMachine<E>>,
    tx: mpsc::SyncSender<ContextEvent<E>>,
    rx: bus::BusReader<ContextEvent<E>>,
) {
    debug!("Thread: started");
    let mut sm = sm;
    let mut rx = rx;
    let mut context = WorkerContext(tx);
    while let Ok(request) = rx.recv() {
        match request {
            ContextEvent::Start => {
                debug!("Thread: Receives start event");
                sm.start(&mut context);
            }
            ContextEvent::Stop => {
                debug!("Thread: Receives stop event");
                break;
            }
            ContextEvent::Envelope(event) => {
                debug!("Thread: Receives event: {:?}", event);
                sm.dispatch(&mut context, event);
            }
        }
    }
    debug!("Finish thread");
}

/// Threaded Context for state machines
///
/// - Each thread runs a state machine
/// - Each thread publishes events, all threads receive the events
///   (multiple producer, multiple consumer)
///   - mpsc is used for multiple producer single consumer
///   - a dispatcher is the single consumer
///   - bus is used for single producer multiple consumer
///   - the events to be distributed need to implement the Clone trait
/// - This multiple producer "all" consumer approach is good enough for this example
///   with limited threads and demonstration purposes only
///
/// # Example
///
/// ```ignore
///
/// let context = ThreadedContext::<u8>::new();
///
/// // Register the state machine threads
/// context.add(StateMachine::<u8>::new("Some State machine on u8 events"));
/// context.add((StateMachine::<u8>::new("Some other State machine on u8 events"))
///
/// // run the state machines
/// context.run();
///
/// ```
pub struct ThreadedContext<E>
where
    E: Debug + Clone + Send + Sync + 'static,
    mpsc::Receiver<ContextEvent<E>>: Send ,
{
    base_tx: mpsc::SyncSender<ContextEvent<E>>,
    mix_rx: Option<mpsc::Receiver<ContextEvent<E>>>,
    mix_tx: Option<bus::Bus<ContextEvent<E>>>,
    threads: Option<Vec<JoinHandle<()>>>,
}

impl <E> ThreadedContext<E>
where
    E: Clone + Debug + Send + Sync + 'static,
    mpsc::Receiver<ContextEvent<E>>: Send ,
{

    pub fn new() -> Self {
        debug!("new: Start state machine runtime context using threads, channels and busses");
        let (base_tx, mix_rx) = mpsc::sync_channel(100); // set up fan-in
        let mix_tx = Bus::new(100); // set up fan-out

        ThreadedContext::<E> {
            base_tx,
            mix_tx: Some(mix_tx),
            mix_rx: Some(mix_rx),
            threads: Some(vec![]),
        }
    }

    pub fn add(&mut self, state_machine: Box< dyn StateMachine<E> + Send>)
    {
        let tx = self.base_tx.clone(); // clone fan in for move to thread
        if let Some(thread) = &mut self.threads {
        if let Some(mix_tx) = &mut self.mix_tx {
            let rx = mix_tx.add_rx(); // register fan out for move to thread
            thread.push(spawn(move || {
                sm_worker(state_machine, tx, rx);
            }));
            debug!("add: State machine thread spawned");
        }}
    }

    pub fn run(&mut self) {
        debug!("run: function invoked");
        // start dispatcher

         // cannot inject mix_rx and mix_tx directly
         // require 'static lifetime so we have to move
        if let Some(mix_rx) = self.mix_rx.take() {
        if let Some(mut mix_tx) = self.mix_tx.take() {
            let _dispatcher = spawn(move || {
                for m in mix_rx.iter() {
                    mix_tx.broadcast(m);
                }
            });

            debug!("run: Message dispatcher thread started");

            // Start all state machines
            self.base_tx.send(ContextEvent::Start).expect("Could not send start event");

            // // Stop all state machines
            let tx = self.base_tx.clone();
            ctrlc::set_handler(
                move || tx.send(ContextEvent::Stop).expect("Could not send stop signal"))
            .expect("Error setting Ctrl-C...");

            // Note: state machines will probably never be stopped
            // are supposed to run forever; panic handling is not implemented on purpose
            if let Some(threads) = self.threads.take() {
                for handle in threads {
                    handle.join().expect("Could not stop thread");
                }
            }

            // Ideally the dispatcher thread needs to be shutdown as well.
            // This would require some extra handling in the dispatcher thread
            // However it will be closed after leaving this scope and the process anyway.
            // Thus, it is not done on purpose
            // _dispatcher.join().unwrap();
        }
    }}
}

