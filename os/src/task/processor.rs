use super::TaskControlBlock;
use alloc::sync::Arc;
use spin::Mutex;
use lazy_static::*;
use super::{fetch_task, TaskStatus};
use super::__switch;
use crate::trap::TrapContext;

pub struct Processor {
    inner: Mutex<ProcessorInner>,
}

unsafe impl Sync for Processor {}

struct ProcessorInner {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx_ptr: usize,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(ProcessorInner {
                current: None,
                idle_task_cx_ptr: 0,
            }),
        }
    }
    fn get_idle_task_cx_ptr2(&self) -> *const usize {
        let inner = self.inner.lock();
        &inner.idle_task_cx_ptr as *const usize
    }
    pub fn run(&self) {
        loop {
            if let Some(task) = fetch_task() {
                let idle_task_cx_ptr = self.get_idle_task_cx_ptr2();
                // acquire
                let next_task_cx_ptr = task.acquire_inner_lock().get_task_cx_ptr2();
                task.acquire_inner_lock().task_status = TaskStatus::Running;
                // release
                self.inner.lock().current = Some(task);
                unsafe {
                    __switch(
                        idle_task_cx_ptr,
                        next_task_cx_ptr,
                    );
                }
            }
        }
    }
    pub fn take_current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.lock().current.take()
    }
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.lock().current.as_ref().map(|task| task.clone())
    }
}

const MAX_CPU_NUM: usize = 2;

lazy_static! {
    pub static ref PROCESSORS: [Processor; MAX_CPU_NUM] = [Processor::new(), Processor::new()];
}

pub fn run_tasks() {
    PROCESSORS[crate::cpu::id()].run();
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSORS[crate::cpu::id()].take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSORS[crate::cpu::id()].current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.acquire_inner_lock().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task().unwrap().acquire_inner_lock().get_trap_cx()
}

pub fn schedule(switched_task_cx_ptr2: *const usize) {
    let idle_task_cx_ptr2 = PROCESSORS[crate::cpu::id()].get_idle_task_cx_ptr2();
    unsafe {
        __switch(
            switched_task_cx_ptr2,
            idle_task_cx_ptr2,
        );
    }
}
