use std::{time, thread};
use std::ops::Sub;

fn main() {
    let target_fps = 60.0;
    let measure_sample = 60;
    let fps_millis = time::Duration::from_secs_f32(1.0 / target_fps);

    let mut measure_timer = time::Instant::now();

    let begin = time::Instant::now();
    for f in 1..i32::MAX {
        let elapsed = begin.elapsed();
        
        // measurement
        if f % measure_sample == 0 {
            let current_fps = measure_sample as f32 / measure_timer.elapsed().as_secs_f32();
            let average_fps = f as f32 / elapsed.as_secs_f32();

            println!("f: {:04}, e:{:.2}s, fps: {:2.2}, fps(avg): {:2.2}",
                f,  elapsed.as_secs_f32(),
                current_fps, average_fps);

            // reset timer
            measure_timer = time::Instant::now();
        }
        
        // wait
        let target_time = fps_millis.mul_f32(f as f32);
        let wait_time = target_time.sub(elapsed);
        thread::sleep(wait_time);
    }
}
