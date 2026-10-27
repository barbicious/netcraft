use hecs::Bundle;

#[derive(Bundle)]
pub struct PositionBundle {
    pub x: f32,
    pub y: f32,
}

pub struct dVelocityBundle {
    pub x: f32,
    pub y: f32,
}