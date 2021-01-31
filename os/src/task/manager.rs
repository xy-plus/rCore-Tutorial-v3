use super::TaskControlBlock;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use hashbrown::HashMap;
use lazy_static::*;
use spin::Mutex;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
    mails: HashMap<usize, VecDeque<[u8; 256]>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            mails: HashMap::new(),
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
    pub fn mail_read(&mut self, pid: usize, buf: &mut [u8]) -> isize {
        if let Some(mail) = self.mails.get_mut(&pid) {
            let tmp_buf = mail.pop_front().unwrap();
            let mut len = buf.len();
            if len > 256 {
                len = 256;
            }
            for i in 0..len {
                buf[i] = tmp_buf[i];
            }
            if (mail.len() == 0) {
                self.mails.remove(&pid);
            }
            return buf.len() as isize;
        } else {
            return -1;
        }
    }
    pub fn mail_write(&mut self, pid: usize, buf: &mut [u8]) -> isize {
        if !self.mails.contains_key(&pid) {
            self.mails.insert(pid, VecDeque::new());
        }
        let mail = self.mails.get_mut(&pid).unwrap();
        if mail.len() >= 16 {
            return -1;
        }
        let mut tmp_buf = [0; 256];
        let mut len = buf.len();
        if len > 256 {
            len = 256;
        }
        for i in 0..len {
            tmp_buf[i] = buf[i];
        }
        mail.push_back(tmp_buf);
        return buf.len() as isize;
        
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.lock().fetch()
}

pub fn mail_read(pid: usize, buf: &mut [u8]) -> isize {
    TASK_MANAGER.lock().mail_read(pid, buf)
}

pub fn mail_write(pid: usize, buf: &mut [u8]) -> isize {
    TASK_MANAGER.lock().mail_write(pid, buf)
}
