use bevy::prelude::Vec2;
use crate::config::Config;

pub fn get_config_from_json(json: &str) -> Config
{
    let config = serde_json::from_str(json).expect("Failed to deserialise config json");
    config
}
pub fn get_config_from_ron(ron: &str) -> Config
{
    let config = ron::de::from_str(ron).expect("Failed to deserialise config ron");
    config
}

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
