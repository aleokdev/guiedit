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
