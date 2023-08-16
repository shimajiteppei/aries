use nes_sdl::start_nes;

fn main() {
    start_nes(String::from("assets/helloworld.nes")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROJECT_ROOT: &str = "../";
    fn start(rel_path: &str) {
        start_nes(String::from(String::from(PROJECT_ROOT) + rel_path)).unwrap();
    }

    #[test]
    fn helloworld() {
        start("assets/helloworld.nes");
    }

    #[test]
    fn nestest() {
        start("assets/nes-test-roms/other/nestest.nes");
    }
}
