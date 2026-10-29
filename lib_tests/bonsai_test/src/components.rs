use paste::paste;

pub type FloatType = f32;
pub type IntType = i32;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: FloatType,
    pub y: FloatType,
    pub z: FloatType,
}
pub trait Vec3NewAdaptor {
    fn to_float(self) -> FloatType;
}
impl Vec3NewAdaptor for IntType {
    fn to_float(self) -> FloatType {
        self as FloatType
    }
}
impl Vec3NewAdaptor for FloatType {
    fn to_float(self) -> FloatType {
        self
    }
}
#[allow(dead_code)]
impl Vec3 {
    pub fn new<T: Vec3NewAdaptor>(x: T, y: T, z: T) -> Self {
        Vec3 {
            x: x.to_float(),
            y: y.to_float(),
            z: z.to_float(),
        }
    }
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn one() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
    pub fn add_vec(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn add_scalar(&self, other: FloatType) -> Self {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
    pub fn add_scalar_int(&self, other: IntType) -> Self {
        self.add_scalar(other as FloatType)
    }
    pub fn sub_vec(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    pub fn sub_scalar(&self, other: FloatType) -> Self {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
    pub fn sub_scalar_int(&self, other: IntType) -> Self {
        self.sub_scalar(other as FloatType)
    }
    pub fn mul_vec(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    pub fn mul_scalar(&self, other: FloatType) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
    pub fn mul_scalar_int(&self, other: IntType) -> Self {
        self.mul_scalar(other as FloatType)
    }
    pub fn div_vec(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
    pub fn div_scalar(&self, other: FloatType) -> Self {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
    pub fn div_scalar_int(&self, other: IntType) -> Self {
        self.div_scalar(other as FloatType)
    }
    pub fn length_squared(&self) -> FloatType {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> FloatType {
        self.length_squared().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            let inv_len = 1.0 / len;
            Vec3 {
                x: self.x * inv_len,
                y: self.y * inv_len,
                z: self.z * inv_len,
            }
        } else {
            Vec3::zero()
        }
    }
    pub fn dot_vec(&self, other: &Self) -> FloatType {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross_vec(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
// impl<'a,'b> std::ops::Add<&'b &Vec3> for &'a &Vec3 {
//     type Output = Vec3;
//     fn add(self, rhs: &'b &Vec3) -> Self::Output {
//         self.add_vec(rhs)
//     }
// }
// impl std::ops::Add< &Vec3> for &Vec3 {
//     type Output = Vec3;
//     fn add(self, rhs: &Vec3) -> Self::Output {
//         self.add_vec(rhs)
//     }
// }
// impl std::ops::Add<Vec3> for Vec3 {
//     type Output = Vec3;
//     fn add(self, rhs: Self) -> Self::Output {
//         self.add_vec(&rhs)
//     }
// }
// impl std::ops::Add<FloatType> for Vec3 {
//     type Output = Self;
//     fn add(self, rhs: FloatType) -> Self {
//         self.add_scalar(rhs)
//     }
// }
macro_rules! ops_define {
    ($ops_name:ident) => {
        paste! {
        impl std::ops::$ops_name< &Vec3> for &Vec3 {
            type Output = Vec3;
            fn [<$ops_name:lower>](self, rhs: &Vec3) -> Self::Output {
                self.[<$ops_name:lower _vec>](rhs)
            }
        }
        impl std::ops::$ops_name<Vec3> for Vec3 {
            type Output = Vec3;
            fn [<$ops_name:lower>](self, rhs: Self) -> Self::Output {
                self.[<$ops_name:lower _vec>](&rhs)
            }
        }
        impl std::ops::$ops_name<FloatType> for Vec3 {
            type Output = Self;
            fn [<$ops_name:lower>](self, rhs: FloatType) -> Self {
                self.[<$ops_name:lower _scalar>](rhs)
            }
        }
        impl std::ops::$ops_name<FloatType> for &Vec3 {
            type Output = Vec3;
            fn [<$ops_name:lower>](self, rhs: FloatType) -> Self::Output {
                self.[<$ops_name:lower _scalar>](rhs)
            }
        }
        impl std::ops::$ops_name<IntType> for Vec3 {
            type Output = Self;
            fn [<$ops_name:lower>](self, rhs: IntType) -> Self {
                self.[<$ops_name:lower _scalar_int>](rhs)
            }
        }
        impl std::ops::$ops_name<IntType> for &Vec3 {
            type Output = Vec3;
            fn [<$ops_name:lower>](self, rhs: IntType) -> Self::Output {
                self.[<$ops_name:lower _scalar_int>](rhs)
            }
        }
                }
    };
}
ops_define!(Add);
ops_define!(Sub);
ops_define!(Mul);
ops_define!(Div);

#[allow(dead_code)]
pub type Pos = Vec3;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3() {
        let zero = Vec3::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
        assert_eq!(zero.z, 0.0);

        let one = Vec3::one();
        let three = one.mul_scalar(3.0);
        assert_eq!(three.x, 3.0);

        let three = one * 3;
        assert_eq!(three.x, 3.0);

        let nine = three
            * Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            };
        assert_eq!(nine.x, 9.0);
        assert_eq!(nine, Vec3::new(9, 9, 9));
    }

    #[test]
    fn test_scalar_operations() {
        let v = Vec3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        assert_eq!(
            v.add_scalar(1.0),
            Vec3 {
                x: 3.0,
                y: 5.0,
                z: 7.0
            }
        );
        assert_eq!(
            v.sub_scalar(1.0),
            Vec3 {
                x: 1.0,
                y: 3.0,
                z: 5.0
            }
        );
        assert_eq!(
            v.mul_scalar(2.0),
            Vec3 {
                x: 4.0,
                y: 8.0,
                z: 12.0
            }
        );
        assert_eq!(
            v.div_scalar(2.0),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_integer_scalar_operations() {
        let v = Vec3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        assert_eq!(
            v.add_scalar_int(2),
            Vec3 {
                x: 4.0,
                y: 6.0,
                z: 8.0
            }
        );
        assert_eq!(
            v.sub_scalar_int(2),
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 4.0
            }
        );
        assert_eq!(
            v.mul_scalar_int(2),
            Vec3 {
                x: 4.0,
                y: 8.0,
                z: 12.0
            }
        );
        assert_eq!(
            v.div_scalar_int(2),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_borrowed_scalar_operations() {
        let v = Vec3::new(8, 12, 16);
        let v_ref = &v;

        assert_eq!(v_ref + 2.0, Vec3::new(10, 14, 18));
        assert_eq!(v_ref - 2.0, Vec3::new(6, 10, 14));
        assert_eq!(v_ref * 2.0, Vec3::new(16, 24, 32));
        assert_eq!(v_ref / 2.0, Vec3::new(4, 6, 8));

        assert_eq!(v_ref + 2, Vec3::new(10, 14, 18));
        assert_eq!(v_ref - 2, Vec3::new(6, 10, 14));
        assert_eq!(v_ref * 2, Vec3::new(16, 24, 32));
        assert_eq!(v_ref / 2, Vec3::new(4, 6, 8));
        assert_eq!(v, Vec3::new(8, 12, 16));
    }
}
