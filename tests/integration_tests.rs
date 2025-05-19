#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, testutils::EnvExt};
    use hello_counter::HelloCounter;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let val = HelloCounter::increment(env.clone());
        assert_eq!(val, 1);
    }
}
