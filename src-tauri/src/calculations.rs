pub fn calculate_scoped_counts(cm_per360: f64, dpi: i32, normal_fov: f64, scoped_fov: f64) -> i32 {
    let normal_fov_radians = std::f64::consts::PI * normal_fov / 180.0;
    let scoped_fov_radians = std::f64::consts::PI * scoped_fov / 180.0;

    // Calculate the number of counts needed for a 360-degree turn
    let inches_per360 = cm_per360 / 2.54;
    let counts_per360 = inches_per360 * dpi as f64;

    // Apply focal length scaling
    let scoped_counts = counts_per360 * ((normal_fov_radians / 2.0).tan() / (scoped_fov_radians / 2.0).tan());
    scoped_counts.round() as i32
}

pub fn calculate_yaw(counts: i32, sens: f64) -> f64 {
    let inc = 360.0 / counts as f64;
    inc / sens
}

pub fn estimate_fov(sens: f64, yaw: f64, counts: i32) -> f64 {
    (yaw * counts as f64 * sens) * 2.0
}

pub fn calculate_counts(cm: f64, dpi: i32) -> i32 {
    let inches_per360 = cm / 2.54;
    let counts_per360 = inches_per360 * dpi as f64;
    counts_per360.round() as i32
}

pub fn calculate_cm(sens: f64, dpi: i32, yaw: f64) -> f64 {
    let counts = 360.0 / (sens * yaw);
    let inches_per360 = counts / dpi as f64;
    let cm_per360 = inches_per360 * 2.54;
    cm_per360
}

pub fn calculate_sens(cm_per360: f64, dpi: i32, yaw: f64) -> f64 {
    let inches_per360 = cm_per360 / 2.54;
    let counts = inches_per360 * dpi as f64;
    let sens = 360.0 / (counts * yaw);
    sens
}

pub fn convert_sensitivity(old_sens: f64, old_dpi: i32, new_dpi: i32, yaw1: f64, yaw2: f64) -> f64 {
    let cm_per360 = calculate_cm(old_sens, old_dpi, yaw1);
    let sens = calculate_sens(cm_per360, new_dpi, yaw2);
    sens
}