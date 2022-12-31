#![no_std]

use gstd::{debug, metadata, prelude::*};
use gdbg::dbg;

pub fn add(left: u32, right: u32) -> u32 {
    // TODO this leads to panic, why?
    // dbg!(left + right)
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[no_mangle]
extern "C" fn init() {
    let msg_bytes = gstd::msg::load_bytes().unwrap(); // read input message bytes
    gstd::debug!("init() is being called with bytes: {:?}", msg_bytes);
}

#[no_mangle]
extern "C" fn handle() {
    let action: Action = gstd::msg::load().expect("failed to load action");
    match action {
        Action::Add(left, right) => {
            dbg!((left, right));
            gstd::msg::reply(Event::Added(dbg!(add(left, right))), 0).expect("failed to reply");
        }
    }
}

metadata! {
    title: "adder",
    handle:
        input: Action,
        output: Event,
}

mod codec {
    use crate::*;
    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    pub mod action {
        use super::*;
        #[derive(Decode, Encode, TypeInfo)]
        #[codec(crate = gstd::codec)]
        #[scale_info(crate = gstd::scale_info)]
        pub enum Action {
            Add(u32, u32),
        }

        #[derive(Decode, Encode, TypeInfo)]
        #[codec(crate = gstd::codec)]
        #[scale_info(crate = gstd::scale_info)]
        pub enum Event {
            Added (u32),
        }
    }
}
use codec::action::{Action, Event};

#[cfg(test)]
use gtest::{Program, System};

#[cfg(test)]
pub fn init_program(prog: &Program) {
    // skip init
    prog.send_bytes(0, "init");
}

#[test]
fn it_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    assert_eq!(program.id(), 1.into());

    let program2 = Program::current(&system);
    assert_eq!(program2.id(), 2.into());

    let program3 = Program::current_with_id(&system, 3);
    assert_eq!(program3.id(), 3.into());

    let program4 = Program::current(&system);
    assert_eq!(program4.id(), 4.into());

    init_program(&program);

    let res = program.send(42, Action::Add(1, 1));
    assert_eq!(res.main_failed(), false);

    let res = program.send(69, Action::Add(2, 2));
    assert_eq!(res.main_failed(), false);

    let res = program.send(1337, Action::Add(7, 9));
    assert_eq!(res.main_failed(), false);
}
