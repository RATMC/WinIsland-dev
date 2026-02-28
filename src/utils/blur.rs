pub fn calculate_blur_sigmas(vel_w: f32, vel_h: f32) -> (f32, f32) {
    let sx = (vel_w.abs() * 0.3).min(10.0);
    let sy = (vel_h.abs() * 0.3).min(10.0);
    (sx, sy)
}
