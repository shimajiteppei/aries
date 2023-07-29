use nes_sdl::start_nes;

fn main() {
    start_nes(String::from("assets/helloworld.nes")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROJECT_ROOT: &str = "../";

    #[test]
    fn helloworld() {
        start_nes(String::from(String::from(PROJECT_ROOT) + "assets/helloworld.nes")).unwrap();
    }

    #[test]
    fn nestest() {
        start_nes(String::from(String::from(PROJECT_ROOT) + "assets/nes-test-roms/other/nestest.nes")).unwrap();
    }

    #[test]
    fn hjoge() {
        start_nes(String::from(String::from(PROJECT_ROOT) + "assets/nes-test-roms/instr_test-v5/official_only.nes")).unwrap();
    }
}
