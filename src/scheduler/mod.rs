// Copyright (c) 2017 Stefan Lankes, RWTH Aachen University
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![allow(dead_code)]
#![allow(private_no_mangle_fns)]

//! Interface to the scheduler

/// task control block
pub mod task;
mod scheduler;

static mut SCHEDULER: scheduler::Scheduler = scheduler::Scheduler::new();

/// Initialite module, must be called once, and only once
pub fn init() {
	unsafe {
		SCHEDULER.add_idle_task();
	}
}

/// Create a new kernel task
#[inline(always)]
pub fn spawn(func: extern fn()) -> Result<task::TaskId, scheduler::SchedulerError> {
	unsafe {
		SCHEDULER.spawn(func)
	}
}

/// Trigger the scheduler to switch to the next available task
#[inline(always)]
pub fn reschedule() {
	unsafe {
		SCHEDULER.reschedule()
	}
}

/// Terminate the current running task
#[inline(always)]
pub fn do_exit() {
	unsafe {
		SCHEDULER.exit();
	}
}

/// Get the TaskID of the current running task
#[inline(always)]
pub fn get_current_taskid() -> task::TaskId {
	unsafe {
		SCHEDULER.get_current_taskid()
	}
}