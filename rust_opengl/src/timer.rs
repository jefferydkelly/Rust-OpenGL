use core::f32;

use crate::traits::Updated;
use once_cell::sync::OnceCell;

static mut TIMER_MANAGER:OnceCell<TimerManager> = OnceCell::new();
struct TimerManager {
    timers:Vec<Timer>,
}

impl TimerManager {
    pub fn create_instance() {
        let timmy = TimerManager {
            timers:Vec::new()
        };
        unsafe {
            TIMER_MANAGER.set(timmy);
        }
    }

    pub fn get_instance()->&'static mut TimerManager {
        unsafe {
            TIMER_MANAGER.get_mut().expect("Timer Manager has not been created")
        }
    }

    pub fn add_timer(&mut self, timer:Timer) {
        self.timers.push(timer);
    }
}

impl Updated for TimerManager {
    fn update(&mut self, dt:f32) {
        for t in &mut self.timers {
            t.update(dt);
        }
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TimerState {
    UNSTARTED,
	RUNNING,
	PAUSED,
    READY,
	STOPPED,
	COMPLETED
}

pub struct Timer {
    tick_time:f32,
    elapsed_time:f32,
    infinite:bool,
    max_ticks:i32,
    times_ticked:i32,
    state:TimerState,
    timed:Vec<Box<dyn Timed>>
}

impl Timer {
    pub fn new()->Timer {
        Timer {
            tick_time:0.0,
            elapsed_time: 0.0,
            infinite:false,
            max_ticks: 0,
            times_ticked:0,
            state:TimerState::UNSTARTED,
            timed:Vec::new()
        }
    }

    pub fn start(&mut self,  tt:f32, mt:i32) {
        if self.state == TimerState::UNSTARTED {
            self.state = TimerState::RUNNING;
            self.tick_time = tt;
            self.elapsed_time = 0.0;
            self.infinite = false;
            self.max_ticks = mt;
            self.times_ticked = 0;
        }
    }

    pub fn start_infinite(&mut self, tt:f32) {
        if self.state == TimerState::UNSTARTED {
            self.tick_time = tt;
            self.elapsed_time = 0.0;
            self.infinite = true;
            self.state = TimerState::RUNNING;
        }
        
    }

    pub fn start_oneshot(&mut self, tt:f32) {
        if self.state == TimerState::UNSTARTED {
            self.tick_time = tt;
            self.elapsed_time = 0.0;
            self.infinite = false;
            self.max_ticks = 1;
            self.times_ticked = 0;
            self.state = TimerState::RUNNING;
        }
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::RUNNING {
            self.state = TimerState::PAUSED;
        }
    }

    pub fn unpause(&mut self) {
        if self.state == TimerState::PAUSED {
            self.state = TimerState::RUNNING;
        }
    }

    pub fn tick(&mut self) {
     
        self.times_ticked+=1;
        self.elapsed_time = 0.0;
       
        if !self.infinite && self.times_ticked >= self.max_ticks {
            self.state = TimerState::COMPLETED;
            for t in self.timed.iter_mut() {
                t.on_complete();
            }
        } else {
            self.state = TimerState::RUNNING;

            for t in self.timed.iter_mut() {
                t.on_tick();
            }
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == TimerState::READY
    }

    pub fn reset(&mut self) {
        self.elapsed_time = 0.0;
        self.times_ticked = 0;
        self.state = TimerState::RUNNING;
    }

    pub fn add_listener(&mut self, the_timed:Box<dyn Timed>) {
        self.timed.push(the_timed);
    }
}

impl Updated for Timer {
    fn update(&mut self, dt:f32) {
       
        if self.state == TimerState::RUNNING {
            self.elapsed_time += dt;

            if self.elapsed_time >= self.tick_time {
               self.state = TimerState::READY;
            }
         }
    }
}

pub trait Timed {

    fn on_tick(&mut self);
    fn on_complete(&mut self);
    fn update_timer(&mut self, dt:f32);
    fn start_timer(&mut self, time:f32, reps:i32);
    
}
