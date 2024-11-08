use xcap::{Monitor, XCapResult};
use std::thread;
use std::time::Duration;

pub fn capture_video(frames: usize, frame_rate: u64) -> XCapResult<Vec<Vec<u8>>> {
    let monitor = Monitor::all()?.into_iter().find(|m| m.is_primary()).unwrap();
    let mut video_frames = Vec::new();

    for _ in 0..frames {
        let frame = monitor.capture_image()?;
        video_frames.push(frame.as_raw().to_vec());
        thread::sleep(Duration::from_millis(1000 / frame_rate));
    }

    Ok(video_frames)
}
