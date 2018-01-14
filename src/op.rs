use std::ops;
use lalg::*;

// Defines operator overload for T x T, &T x T, T x &T and &T x &T
macro_rules! lalg_op {
    ($op_trait:tt, $op_func:ident, ($name_a:ident : $type_a:ty, $name_b:ident : $type_b:ty) -> $result_type:ty $code:block) => {
        impl ops::$op_trait<$type_b> for $type_a {
            type Output = $result_type;
            fn $op_func($name_a, $name_b : $type_b) -> $result_type {
                $code
            }
        }
        impl<'a> ops::$op_trait<$type_b> for &'a $type_a {
            type Output = $result_type;
            fn $op_func($name_a, $name_b : $type_b) -> $result_type {
                $code
            }
        }
        impl<'a> ops::$op_trait<&'a $type_b> for $type_a {
            type Output = $result_type;
            fn $op_func($name_a, $name_b : &'a $type_b) -> $result_type {
                $code
            }
        }
        impl<'a, 'b> ops::$op_trait<&'a $type_b> for &'b $type_a {
            type Output = $result_type;
            fn $op_func($name_a, $name_b : &'a $type_b) -> $result_type {
                $code
            }
        }
    };
}

lalg_op!{ Add, add, (self:Vec4, other:Vec4) -> Vec4 {
    Vec4([self.0[0] + other.0[0], self.0[1] + other.0[1], self.0[2] + other.0[2], self.0[3] + other.0[3]])
} }
lalg_op!{ Add, add, (self:Vec3, other:Vec3) -> Vec3 {
    Vec3([self.0[0] + other.0[0], self.0[1] + other.0[1], self.0[2] + other.0[2]])
} }
lalg_op!{ Add, add, (self:Vec2, other:Vec2) -> Vec2 {
    Vec2([self.0[0] + other.0[0], self.0[1] + other.0[1]])
} }


lalg_op!{ Sub, sub, (self:Vec4, other:Vec4) -> Vec4 {
    Vec4([self.0[0] - other.0[0], self.0[1] - other.0[1], self.0[2] - other.0[2], self.0[3] - other.0[3]])
} }
lalg_op!{ Sub, sub, (self:Vec3, other:Vec3) -> Vec3 {
    Vec3([self.0[0] - other.0[0], self.0[1] - other.0[1], self.0[2] - other.0[2]])
} }
lalg_op!{ Sub, sub, (self:Vec2, other:Vec2) -> Vec2 {
    Vec2([self.0[0] - other.0[0], self.0[1] - other.0[1]])
} }


lalg_op!{ Mul, mul, (self:Vec4, other:f32) -> Vec4 {
    Vec4([self.0[0] * other, self.0[1] * other, self.0[2] * other, self.0[3] * other])
} }
lalg_op!{ Mul, mul, (self:Vec3, other:f32) -> Vec3 {
    Vec3([self.0[0] * other, self.0[1] * other, self.0[2] * other])
} }
lalg_op!{ Mul, mul, (self:Vec2, other:f32) -> Vec2 {
    Vec2([self.0[0] * other, self.0[1] * other])
} }

lalg_op!{ Div, div, (self:Vec4, other:f32) -> Vec4 {
    Vec4([self.0[0] / other, self.0[1] / other, self.0[2] / other, self.0[3] / other])
} }
lalg_op!{ Div, div, (self:Vec3, other:f32) -> Vec3 {
    Vec3([self.0[0] / other, self.0[1] / other, self.0[2] / other])
} }
lalg_op!{ Div, div, (self:Vec2, other:f32) -> Vec2 {
    Vec2([self.0[0] / other, self.0[1] / other])
} }

lalg_op!{ Mul, mul, (self:Mat4x4, vec:Vec4) -> Vec4 {
    Vec4([ Vec4::dot(&self.row(0), &vec),
           Vec4::dot(&self.row(1), &vec),
           Vec4::dot(&self.row(2), &vec),
           Vec4::dot(&self.row(3), &vec)])
} }

lalg_op!{ Mul, mul, (self:Vec4, mat:Mat4x4) -> Vec4 {
    Vec4([Vec4::dot(&self, &mat.col(0)),
          Vec4::dot(&self, &mat.col(1)),
          Vec4::dot(&self, &mat.col(2)),
          Vec4::dot(&self, &mat.col(3))])
} }

lalg_op!{ Add, add, (self:Mat4x4, other:Mat4x4) -> Mat4x4 {
    collect_mat(&mut self.0.iter().zip(other.0.iter()).map(|(a,b)| a+b))
} }

lalg_op!{ Sub, sub, (self:Mat4x4, other:Mat4x4) -> Mat4x4 {
    collect_mat(&mut self.0.iter().zip(other.0.iter()).map(|(a,b)| a-b))
} }

lalg_op!{ Mul, mul, (self:Mat4x4, other:Mat4x4) -> Mat4x4 {
    let m0 = &self;
    let m1 = &other;
    collect_mat( &mut (0..16).map(|i| {
        let (y,x) = (i/4,i%4);
        Vec4::dot(&m0.row(y), &m1.col(x))
        } ) )
 } }

