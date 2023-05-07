use rand::seq::SliceRandom;

pub fn get_random_gymnastic() -> String {
    // Resources:
    // * https://github.com/slgobinath/SafeEyes/blob/master/safeeyes/config/safeeyes.json#L20
    // * ChatGPT, internet ...
    let data: Vec<&str> = vec![
        "Take a brief walk and hydrate with water 💧",
        "Blink your eyes 😉",
        "Close your eyes and envision a peaceful place 🏞️",
        "Connect with a loved one through a phone call or text message 📱",
        "Create a gratitude list to appreciate your blessings 📝",
        "Do a cardiac coherence session ❤️‍🩹",
        "Engage in a creative activity, such as drawing or painting 🎨",
        "Feel the texture of an object in your hands, grounding you 🖐️",
        "Focus on a point in the far distance 🌅",
        "Focus on the present moment, letting go of worries 🌈",
        "Gently apply a soothing hand cream for soft, relaxed skin 🤲",
        "Gently stretch your muscles to release tension 🧘",
        "Give yourself a gentle facial massage to release tension 😊",
        "Imagine waves gently crashing on a serene beach 🌊",
        "Inhale for four counts, hold for four, exhale for four 🌬️",
        "Inhale the aroma of lavender to soothe your nerves 🌿",
        "Laugh out loud by watching a funny video or reading jokes 😂",
        "Lean back at your seat and relax 💺",
        "Light a scented candle to create a soothing ambiance 🕯️",
        "Listen to an inspiring podcast or audiobook 🎧",
        "Listen to calming music or nature sounds 🎶",
        "Meditate for a few minutes to clear your mind 🧘",
        "Perform a random act of kindness to boost your mood 💕",
        "Perform gentle yoga poses to increase flexibility and balance 🧘🏼",
        "Perform hand and arm stretches to release muscle tension 💪",
        "Picture your worries as balloons, floating away 🎈",
        "Plant or tend to a garden, connecting with nature 🌱",
        "Practice mindfulness by focusing on the present moment 🌸",
        "Practice progressive muscle relaxation, tensing and releasing muscles 👣",
        "Read a few pages of a comforting book 📖",
        "Repeat a calming mantra, such as 'I am at peace.' ☮️",
        "Roll your eyes a few times to each side 👀",
        "Rotate your eyes in clockwise direction 🔄",
        "Rotate your eyes in counterclockwise direction 🔄",
        "Savor a delicious, healthy snack 🍇",
        "Savor a warm cup of tea or a soothing scent ☕",
        "Sip a warm, calming beverage, like herbal tea ☕",
        "Slowly roll your shoulders back and release tension 🦺",
        "Soak up the sun and get a dose of Vitamin D ☀",
        "Stretch your legs 🚶️",
        "Take a deep breath and let it out slowly 🌬️",
        "Take a nap or rest, allowing your body to rejuvenate 💤",
        "Take a relaxing bath with bubbles or bath salts 🛀",
        "Take slow, deep breaths and focus on your breathing 🌬️",
        "Tightly close your eyes 😌",
        "Visualize your stress melting away like ice ❄",
        "Walk for a while 🚶️",
        "Whisper to yourself, 'I am calm and in control' 😊",
        "Wrap yourself in a cozy blanket and relax on the couch 🛋️",
        "Write in a journal to express your thoughts and emotions 🖋",
    ]
    .iter()
    .cloned()
    .map(|s| s.trim())
    .collect();

    data.choose(&mut rand::thread_rng())
        .unwrap_or(&"Take a pause and debug me 😏")
        .to_string()
}
