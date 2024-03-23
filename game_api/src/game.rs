struct Menu;

impl Menu {
    fn screen_option(self) -> ChooseMode {
        todo!()
    }
}

struct ChooseMode;
struct Game;

pub struct WorldLogic<T>(T);


impl WorldLogic<Menu> {
    fn start(self) -> WorldLogic<ChooseMode> {
        WorldLogic(self.0.screen_option())
    }
}