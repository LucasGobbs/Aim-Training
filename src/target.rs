use rlua::{UserData, UserDataMethods};

#[derive(Copy, Clone)]
pub struct Target{
    pub x: f32,
    pub y: f32,
    radius: i32,
    life: i32,
    is_dead: bool,
}
impl Target{
    pub fn new(_x: f32, _y: f32) -> Target {
        Target{
            x: _x,
            y: _y,
            radius: 80,
            life: 100,
            is_dead: false,
        }
    }
    pub fn is_inside(self, _x: f32, _y: f32) -> bool{
        let dist_sqr = (self.x - _x) * (self.x - _x) + (self.y - _y)*(self.y - _y);
        return dist_sqr.sqrt() <= self.radius as f32;
    }
    //pub const lua_constructor: fn(_,(f32, f32)) -> Some(Target) = |_, (x, y): (f32, f32)| Ok(Target::new(x,y));
}
impl UserData for Target {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("isInside", |_, target, (x,y): (f32,f32)| {
            let flag = target.is_inside(x,y);
            Ok(flag)
        });
        methods.add_method("x", |_, target, ()| {
            Ok(target.x)
        });
        methods.add_method("y", |_, target, ()| {
            Ok(target.y)
        });
        methods.add_method("remove", |_, target, ()| {
            let mut other_target: Target = target.clone();
            other_target.is_dead = true;
            other_target.life = 0;
            Ok(other_target)
        });
    }
}