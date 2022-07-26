use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_us;
use crate::task::task::TaskStatus;
use crate::task::{get_already_run_time,get_syscall_times};
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_get_time(ts:*mut TimeVal, _tv:usize) ->isize{
    let us: usize = get_time_us();
    unsafe {
        *ts = TimeVal{
            sec:us/1_000_000,
            usec:us%1_000_000,
        };
    }
    0
}

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    unsafe {
        *ti = TaskInfo{
            status:TaskStatus::Running,
            syscall_times:get_syscall_times(),
            time:get_already_run_time()
        }
    }
    0
}

#[repr(C)]
pub struct TimeVal{
    pub sec:usize,
    pub usec:usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}