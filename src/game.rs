use rlua::{UserData, UserDataMethods};

use crate::target::Target;
#[derive(Clone)]
pub struct Game{
    pub targets: Vec<Target>,
    pub w_width: i32,
    pub w_height: i32,
}
impl Game{
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            targets: Vec::new(),
            w_height: height,
            w_width: width,
        }
    }
    pub fn add_target(&mut self, new_target: Target){
        self.targets.push(new_target);
    }
}
impl UserData for Game {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("test", |_, _game, ()| {
            println!("Test for Game Struct");
            Ok(())
        });
        methods.add_method("get_target", |_, game, (index,): (usize,)| {
            let target = *game.targets.get(index).unwrap();
            Ok((target,))
        });
        methods.add_method("add", |_, game, (new_t,): (Target,)| {
            let mut other_game = game.clone();
            other_game.add_target(new_t);
            Ok((other_game,))
        });
        methods.add_method("targets", |_, game, ()| {
            let targets = game.clone().targets;
            Ok((targets,))
        });
        methods.add_method("len", |_, game, ()| {
            let targets_len = game.clone().targets.len();
            Ok((targets_len,))
        });
        methods.add_method("width", |_, game, ()| {
            let width = game.clone().w_width;
            Ok((width,))
        });
        methods.add_method("height", |_, game, ()| {
            let height = game.clone().w_height;
            Ok((height,))
        });
        
    }
}