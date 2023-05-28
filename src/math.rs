#[inline(always)]
pub(crate) fn ceil(val: f32) -> f32 {
    let floor = val as u32 as f32;

    if val == floor {
        val
    } else if val > 0.0 {
        floor + 1.0
    } else {
        floor
    }
}

#[inline(always)]
pub(crate) fn round(val: f32) -> f32 {
    let floor = val as u32 as f32;

    if val - floor < 0.5 {
        floor
    } else {
        floor + 1.0
    }
}
