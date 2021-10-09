use std::time::{Instant, Duration};
use spin_sleep::SpinSleeper;


pub struct TickLoopInfo {
    pub tick_duration: Duration
}


/// Run the given function at a stable frequency. This function will block
/// and loop while the given function return true.
pub fn tick_loop<F>(mut func: F, frequency: f32)
where
    F: FnMut(&mut TickLoopInfo) -> bool
{

    let mut info = TickLoopInfo {
        tick_duration: Duration::from_secs_f32(1.0 / frequency)
    };

    let sleeper = SpinSleeper::default();

    loop {

        let start = Instant::now();

        if !func(&mut info) {
            break;
        }

        let elapsed = start.elapsed();
        if elapsed > info.tick_duration {
            println!("Tick took too long: {:?} (expected < {:?})", elapsed, info.tick_duration);
        } else {
            sleeper.sleep(info.tick_duration - elapsed);
        }

    }

}
