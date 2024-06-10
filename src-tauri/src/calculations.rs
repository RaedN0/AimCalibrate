pub fn calculate_scoped_counts(cm_per360: f32, dpi: i32, normal_fov: f32, scoped_fov: f32) -> i32 {
    let normal_fov_radians = std::f32::consts::PI * normal_fov / 180.0;
    let scoped_fov_radians = std::f32::consts::PI * scoped_fov / 180.0;

    // Calculate the number of counts needed for a 360-degree turn
    let inches_per360 = cm_per360 / 2.54;
    let counts_per360 = inches_per360 * dpi as f32;

    // Apply focal length scaling
    let scoped_counts = counts_per360 * ((normal_fov_radians / 2.0).tan() / (scoped_fov_radians / 2.0).tan());
    scoped_counts as i32
}

pub fn calculate_yaw(counts: i32, sens: f32) -> f32 {
    let inc = 360.0 / counts as f32;
    inc / sens
}

pub fn estimate_fov(sens: f32, yaw: f32, counts: i32) -> f32 {
    (yaw * counts as f32 * sens) * 2.0
}

pub fn calculate_counts(cm: f32, dpi: i32) -> i32 {
    let inches_per360 = cm / 2.54;
    let counts_per360 = inches_per360 * dpi as f32;
    counts_per360 as i32
}
