pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = Vec::new();
    let actions = ["wink", "double blink", "close your eyes", "jump"];

    // Check each bit from right to left
    if n & 16 == 16 {
        // Reverse the order of actions
        for (i, &action) in actions.iter().enumerate() {
            if n & (1 << i) != 0 {
                result.insert(0, action);
            }
        }
    } else {
        // Normal order of actions
        for (i, &action) in actions.iter().enumerate() {
            if n & (1 << i) != 0 {
                result.push(action);
            }
        }
    }

    result
}
