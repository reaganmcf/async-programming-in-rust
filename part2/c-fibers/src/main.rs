#![feature(naked_functions)]

use std::arch::asm;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 9;

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Available,
    Running,
    Ready
}

struct Thread {
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64
}

fn main() {
   
}
