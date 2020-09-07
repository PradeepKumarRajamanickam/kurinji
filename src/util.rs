use bevy::prelude::Vec2;

pub(crate) fn normalised_within_range(min: f32, max: f32, value: f32) -> f32 {
    // src: https://stats.stackexchange.com/questions/70801/how-to-normalize-data-to-0-1-range
    // formula: newvalue= (max'-min')/(max-min)*(value-max)+max'
    // new arbitrary range min' to max'
    let r = 1. / (max - min) * (value - max) + 1.0;
    clamp(0., 1., r)
}

pub(crate) fn clamp(min: f32, max: f32, value: f32) -> f32
{
    if value < min {
        return min
    } else if value > max {
        return max
    }
    value
}

pub(crate) fn clamp_vec2(min: Vec2, max: Vec2, value: Vec2) -> Vec2
{
    Vec2::new(
        clamp(min.x(), max.x(), value.x()),
        clamp(min.y(), max.y(), value.y())
    )
}
