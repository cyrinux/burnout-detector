use rand::seq::SliceRandom;

pub fn get_random_gymnastic() -> String {
    // https://github.com/slgobinath/SafeEyes/blob/master/safeeyes/config/safeeyes.json#L20
    let data: Vec<&str> = vec![
        "Tightly close your eyes",
        "Roll your eyes a few times to each side",
        "Rotate your eyes in clockwise direction",
        "Rotate your eyes in counterclockwise direction",
        "Blink your eyes",
        "Focus on a point in the far distance",
        "And have some water",
        "Walk for a while",
        "Lean back at your seat and relax",
        "Do a cardiac coherence session",
    ];

    data.choose(&mut rand::thread_rng())
        .unwrap_or(&"Take a pause and debug me :)")
        .to_string()
}
