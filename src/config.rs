struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub fn load_env_var(name: &'static str) -> String {
    dotenvy::var(name).expect(&format!("missing required env variable: {}", name))
}
