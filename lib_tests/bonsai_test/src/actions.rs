use crate::components::{FloatType, Pos, Vec3};
use crate::entities::Charactor;

pub fn actor_move(delta_time: FloatType, pos: &Pos, speed: &Vec3) -> Pos {
    // 这两个等价
    // pos.add_vec(&(speed.mul_scalar(delta_time)));
    pos + &(speed * delta_time)
}

