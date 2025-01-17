use std::thread;
use std::time::{Duration, Instant};

pub struct FpsCounter
{
    target_fps: usize,
    frame_duration: Duration,
    frame_count: usize,
    last_frame_time: Instant,
    last_fps_update: Instant,

    average_fps: [usize; 5],
    last_index: usize,

    delta_time: Duration
}

impl FpsCounter 
{
    pub fn new(target_fps: usize) -> FpsCounter
    {
        FpsCounter
        {
            target_fps,
            frame_duration: Duration::from_secs_f64(1.0 / target_fps as f64),
            frame_count: 0,
            last_frame_time: Instant::now(),
            last_fps_update: Instant::now(),
            average_fps: [0; 5],
            last_index: 0,
            delta_time: Duration::new(0, 0)
        }
    }

    pub fn frame(&mut self, is_limited: bool) 
    {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame_time);
    
        let designated_time = self.frame_duration * (self.frame_count + 2) as u32;
        let current_relative_time = self.last_fps_update.elapsed();

        if is_limited && designated_time > current_relative_time
        {
            thread::sleep(designated_time - current_relative_time);
        }
    
        self.last_frame_time = Instant::now();
    
        self.frame_count += 1;
    
        if current_relative_time >= Duration::from_secs(1) 
        {
            self.average_fps[self.last_index] = self.frame_count;
            self.last_index = (self.last_index + 1) % self.average_fps.len();
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
        }
    }
    
    
    pub fn set_target_fps(&mut self, target_fps: usize) 
    {
        self.target_fps = target_fps;
        self.frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
    }
    
    pub fn average_fps(&self) -> f32 {
        self.average_fps.iter().fold(0.0, |acc, &num| acc as f32 + num as f32 / self.average_fps.len() as f32) 
    }
    
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }
    
}