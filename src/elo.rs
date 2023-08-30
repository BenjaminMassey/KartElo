static K: f32 = 20f32;

fn expected_elo(rating_a: f32, rating_b: f32) -> f32 {
    1f32 / (1f32 + f32::powf(10f32, (rating_b - rating_a) / 400f32))
}

pub fn elo(rating_a: f32, rating_b: f32, score_a: f32, score_b: f32) -> (f32, f32) {
    assert!((0f32..=1f32).contains(&score_a), "score_a was not in 0-1 range");
    assert!((0f32..=1f32).contains(&score_b), "score_b was not in 0-1 range");
    (
        rating_a + K * (score_a - expected_elo(rating_a, rating_b)),
        rating_b + K * (score_b - expected_elo(rating_b, rating_a))
    )
}