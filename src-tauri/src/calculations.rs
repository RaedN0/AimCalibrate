pub fn calculate_scoped_counts(cm_per360: f64, dpi: i32, normal_fov: f64, scoped_fov: f64) -> f64 {
    let normal_fov_radians = std::f64::consts::PI * normal_fov / 180.0;
    let scoped_fov_radians = std::f64::consts::PI * scoped_fov / 180.0;

    // Calculate the number of counts needed for a 360-degree turn
    let inches_per360 = cm_per360 / 2.54;
    let counts_per360 = inches_per360 * dpi as f64;

    // Apply focal length scaling
    let scoped_counts = counts_per360 * (normal_fov_radians.tan() / scoped_fov_radians.tan());
    scoped_counts
}

pub fn calculate_yaw(counts: i32, sens: f64) -> f64 {
    let inc = 360.0 / counts as f64;
    inc / sens
}

pub fn estimate_fov(sens: f64, yaw: f64, counts: i32) -> f64 {
    (yaw * counts as f64 * sens) * 2.0
}
