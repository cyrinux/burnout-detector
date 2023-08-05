use rand::seq::SliceRandom;

pub fn get_random_gymnastic() -> (String, Option<String>) {
    // Resources:
    // * https://github.com/slgobinath/SafeEyes/blob/master/safeeyes/config/safeeyes.json#L20
    // * ChatGPT, internet ...
    let data: Vec<(String, Option<String>)> = vec![
        ("Take a brief walk and hydrate with water 💧", None),
        ("Blink your eyes 😉", None),
        ("Close your eyes and envision a peaceful place 🏞️", None),
        (
            "Connect with a loved one through a phone call or text message 📱",
            None,
        ),
        (
            "Create a gratitude list to appreciate your blessings 📝",
            None,
        ),
        ("Do a cardiac coherence session ❤️‍🩹", None),
        (
            "Engage in a creative activity, such as drawing or painting 🎨",
            None,
        ),
        (
            "Feel the texture of an object in your hands, grounding you 🖐️",
            None,
        ),
        ("Focus on a point in the far distance 🌅", None),
        (
            "Focus on the present moment, letting go of worries 🌈",
            None,
        ),
        (
            "Gently apply a soothing hand cream for soft, relaxed skin 🤲",
            None,
        ),
        ("Gently stretch your muscles to release tension 🧘", None),
        (
            "Give yourself a gentle facial massage to release tension 😊",
            None,
        ),
        ("Imagine waves gently crashing on a serene beach 🌊", None),
        (
            "Inhale for four counts, hold for four, exhale for four 🌬",
            Some("https://google.com".to_string()),
        ),
        (
            "Inhale the aroma of lavender to soothe your nerves 🌿",
            None,
        ),
        (
            "Laugh out loud by watching a funny video or reading jokes 😂",
            None,
        ),
        ("Lean back at your seat and relax 💺", None),
        (
            "Light a scented candle to create a soothing ambiance 🕯️",
            None,
        ),
        ("Listen to an inspiring podcast or audiobook 🎧", None),
        ("Listen to calming music or nature sounds 🎶", None),
        ("Meditate for a few minutes to clear your mind 🧘", None),
        (
            "Perform a random act of kindness to boost your mood 💕",
            None,
        ),
        (
            "Perform gentle yoga poses to increase flexibility and balance 🧘🏼",
            None,
        ),
        (
            "Perform hand and arm stretches to release muscle tension 💪",
            None,
        ),
        ("Picture your worries as balloons, floating away 🎈", None),
        ("Plant or tend to a garden, connecting with nature 🌱", None),
        (
            "Practice mindfulness by focusing on the present moment 🌸",
            None,
        ),
        (
            "Practice progressive muscle relaxation, tensing and releasing muscles 👣",
            None,
        ),
        ("Read a few pages of a comforting book 📖", None),
        ("Repeat a calming mantra, such as 'I am at peace.' ☮️", None),
        ("Roll your eyes a few times to each side 👀", None),
        ("Rotate your eyes in clockwise direction 🔄", None),
        ("Rotate your eyes in counterclockwise direction 🔄", None),
        ("Savor a delicious, healthy snack 🍇", None),
        ("Savor a warm cup of tea or a soothing scent ☕", None),
        ("Sip a warm, calming beverage, like herbal tea ☕", None),
        (
            "Slowly roll your shoulders back and release tension 🦺",
            None,
        ),
        ("Soak up the sun and get a dose of Vitamin D ☀", None),
        ("Stretch your legs 🚶", None),
        ("Take a deep breath and let it out slowly 🌬️", None),
        (
            "Take a nap or rest, allowing your body to rejuvenate 💤",
            None,
        ),
        ("Take a relaxing bath with bubbles or bath salts 🛀", None),
        (
            "Take slow, deep breaths and focus on your breathing 🌬",
            None,
        ),
        ("Tightly close your eyes 😌", None),
        ("Visualize your stress melting away like ice ❄", None),
        ("Walk for a while 🚶", None),
        ("Whisper to yourself, 'I am calm and in control' 😊", None),
        (
            "Wrap yourself in a cozy blanket and relax on the couch 🛋️",
            None,
        ),
        (
            "Write in a journal to express your thoughts and emotions 🖋",
            None,
        ),
    ]
    .iter()
    .cloned()
    .map(|(s, x)| (s.trim().to_string(), x))
    .collect();

    data.choose(&mut rand::thread_rng()).unwrap().clone()
}
