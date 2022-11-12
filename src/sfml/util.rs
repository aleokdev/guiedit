use sfml::system::Vector2f;

pub fn map(
    value: Vector2f,
    min1: Vector2f,
    max1: Vector2f,
    min2: Vector2f,
    max2: Vector2f,
) -> Vector2f {
    min2 + (value - min1).cwise_mul(max2 - min2).cwise_div(max1 - min1)
}

/// Fits an object with a certain aspect ratio into a rect with the size given, and returns its final size.
pub fn fit_aspect_ratio_in_size(
    aspect_ratio: f32,
    available_size: egui::Vec2,
) -> egui::Vec2 {
    if available_size.y > available_size.x / aspect_ratio
        && available_size.x < available_size.y * aspect_ratio
    {
        // Width-controlled
        egui::Vec2::new(available_size.x, available_size.x / aspect_ratio)
    } else {
        // Height-controlled
        egui::Vec2::new(available_size.y * aspect_ratio, available_size.y)
    }
}
