mod target;
use target::Target;

mod game;
use game::Game;

use std::fs;


use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Event, State, time, window, input};

use rlua::{
    Lua,

    Function,
};
use rand::Rng;


struct GameState {
    file: Text,
    lua: Lua,
    pub current_script: String,
    target_texture: Texture,
    
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let game = Game::new(window::get_height(ctx),window::get_width(ctx));
        let texture = Texture::new(ctx, "./resources/target.png")?;
        let state = GameState {
            file: Text::new(
                "Drop a file onto this window to view the contents.",
                Font::vector(ctx, "./resources/RobotY.ttf", 24.0)?,
            ),
            lua: Lua::new(),
            current_script: fs::read_to_string("src/scripts/basic.lua").unwrap(),
            target_texture: texture,
        };
        state.lua.context(|lua_ctx|{
            let globals = lua_ctx.globals();
            globals.set("game", game).unwrap();

            let target_constructor = lua_ctx.create_function(|_, (x, y): (f32, f32)| Ok(Target::new(x,y))).unwrap();   
            globals.set("target", target_constructor).unwrap();

            let rand = lua_ctx.create_function(|_,(max,): (i32,)|{
                let mut rng = rand::thread_rng();
                let random_number = rng.gen_range(0, max);
                Ok(random_number)
            }).unwrap();
            globals.set("rand_range",rand).unwrap();

            
            lua_ctx.load(&state.current_script)
                   .exec()
                   .unwrap();

            let init_f: Function = globals.get("init").unwrap();
            init_f.call::<_, ()>((),).unwrap();
        });
        Ok(state)
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.lua.context(|lua_ctx| {
            
            let globals = lua_ctx.globals();
            let game = globals.get::<_, Game>("game").unwrap(); 

            let update_f: Function = globals.get("update").unwrap();
            update_f.call::<_, ()>((),).unwrap();

            let on_mouse_hit_target_f: Function = globals.get("on_mouse_hit_target").unwrap();
            let (mx, my) = (input::get_mouse_x(ctx), input::get_mouse_y(ctx));
            for (index,target) in game.targets.iter().enumerate() {
                
                let flag = target.is_inside(mx,my);
                if flag {
                    on_mouse_hit_target_f.call::<_, ()>((index,),).unwrap();
                }
            }
        });
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.file.set_content(format!("fps: {}",time::get_fps(ctx)));
        graphics::draw(ctx, &self.file, Vec2::new(16.0, 16.0));


        self.lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

           
            let game = globals.get::<_, Game>("game").unwrap();
            
            for target in game.targets {
                graphics::draw(ctx, &self.target_texture, Vec2::new(target.x,target.y));
            }
        
        });

        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {

        if let Event::Resized { width, height } = event {
            self.lua.context(|lua_ctx| {
            
                let globals = lua_ctx.globals();

                let mut game = globals.get::<_, Game>("game").unwrap();
                game.w_width = width;
                game.w_height = height;
                globals.set("game", game).unwrap();
            
            });
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("File Dropping", 1280, 720)
        .resizable(true)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}