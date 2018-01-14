use std::f32;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec4(pub [f32;4]);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3(pub [f32;3]);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2(pub [f32;2]);

#[derive(PartialEq, Debug, Clone)]
pub struct Mat4x4(pub [f32;16]);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mat2x2(pub [f32;4]);

impl Vec4 {
    pub fn dot(a : &Vec4, b : &Vec4) -> f32 {
        a.0[0] * b.0[0] + a.0[1] * b.0[1] + a.0[2] * b.0[2] + a.0[3] * b.0[3]
    }
    pub fn len_sq(&self) -> f32 {
        let v = self.0;
        v[0]*v[0]+v[1]*v[1]+v[2]*v[2]+v[3]*v[3]
    }
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3([self.0[0], self.0[1], self.0[2]])
    }

    pub fn x(&self) -> f32 { self.0[0] }
    pub fn y(&self) -> f32 { self.0[1] }
    pub fn z(&self) -> f32 { self.0[2] }
    pub fn w(&self) -> f32 { self.0[3] }


}

impl Vec3 {
    pub fn dot(a : &Vec3, b : &Vec3) -> f32 {
        a.0[0] * b.0[0] + a.0[1] * b.0[1] + a.0[2] * b.0[2]
    }

    pub fn cross(a : &Vec3, b : &Vec3) -> Vec3 {
        Vec3( [Mat2x2([a.y(), a.z(), b.y(), b.z()]).det(), -Mat2x2([a.x(), a.z(), b.x(), b.z()]).det(), Mat2x2([a.x(), a.y(), b.x(), b.y()]).det()] )
    }
    pub fn len_sq(&self) -> f32 {
        let v = self.0;
        v[0]*v[0]+v[1]*v[1]+v[2]*v[2]
    }
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }
    pub fn normalized(&self) -> Vec3 {
        self / self.len()
    }

    pub fn x(&self) -> f32 { self.0[0] }
    pub fn y(&self) -> f32 { self.0[1] }
    pub fn z(&self) -> f32 { self.0[2] }

    pub fn xy(&self) -> Vec2 { Vec2([self.x(), self.y()]) }
    pub fn yz(&self) -> Vec2 { Vec2([self.y(), self.z()]) }
    pub fn xz(&self) -> Vec2 { Vec2([self.x(), self.z()]) }
    pub fn yx(&self) -> Vec2 { Vec2([self.y(), self.x()]) }
    pub fn zy(&self) -> Vec2 { Vec2([self.z(), self.y()]) }
    pub fn zx(&self) -> Vec2 { Vec2([self.z(), self.x()]) }

}

impl Vec2 {
    pub fn dot(a : &Vec2, b : &Vec2) -> f32 {
        a.0[0] * b.0[0] + a.0[1] * b.0[1]
    }
    pub fn len_sq(&self) -> f32 {
        let v = self.0;
        v[0]*v[0]+v[1]*v[1]
    }
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }
    pub fn normalized(&self) -> Vec2 {
        self / self.len()
    }

    pub fn x(&self) -> f32 { self.0[0] }
    pub fn y(&self) -> f32 { self.0[1] }
}

impl From<[f32;2]> for Vec2 {
    fn from(a: [f32;2]) -> Self {
        Vec2(a)
    }
}

impl From<[f32;3]> for Vec3 {
    fn from(a: [f32;3]) -> Self {
        Vec3(a)
    }
}

impl From<[f32;4]> for Vec4 {
    fn from(a: [f32;4]) -> Self {
        Vec4(a)
    }
}

impl Mat2x2 {
    pub fn det(&self) -> f32 {
        self.0[0] * self.0[3] - self.0[1] * self.0[2]
    }
}

impl Mat4x4 {
    pub fn identity() -> Mat4x4 {
        Mat4x4([1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0])
    }

    pub fn scale(s : f32) -> Mat4x4 {
        Mat4x4([  s, 0.0, 0.0, 0.0,
                0.0,   s, 0.0, 0.0,
                0.0, 0.0,   s, 0.0,
                0.0, 0.0, 0.0, 1.0 ])
    }

    pub fn rot_x(amount_rad : f32) -> Mat4x4 {
        let (c,s) = (amount_rad.cos(), amount_rad.sin());
        Mat4x4([ 1.0, 0.0, 0.0, 0.0,
                     0.0,   c,   s, 0.0,
                     0.0,  -s,   c, 0.0,
                     0.0, 0.0, 0.0, 1.0])
    }

    pub fn rot_y(amount_rad : f32) -> Mat4x4 {
        let (c,s) = (amount_rad.cos(), amount_rad.sin());
        Mat4x4([   c, 0.0,  -s, 0.0,
                     0.0, 1.0, 0.0, 0.0,
                       s, 0.0,   c, 0.0,
                     0.0, 0.0, 0.0, 1.0])
    }

    pub fn rot_z(amount_rad : f32) -> Mat4x4 {
        let (c,s) = (amount_rad.cos(), amount_rad.sin());
        Mat4x4([ c,   s, 0.0, 0.0,
                    -s,   c, 0.0, 0.0,
                     0.0, 0.0, 1.0, 0.0,
                     0.0, 0.0, 0.0, 1.0])
    }

    pub fn translate(s : &Vec3) -> Mat4x4 {
        Mat4x4(   [ 1.0,    0.0,    0.0,    0.0,
                    0.0,    1.0,    0.0,    0.0,
                    0.0,    0.0,    1.0,    0.0,
                    s.0[0], s.0[1], s.0[2], 1.0])
    }

    pub fn proj_perspective(aspect : f32, fov_y_angles : f32, z_near : f32, z_far : f32) -> Mat4x4 {
        let fov_y = fov_y_angles / 180.0f32 * f32::consts::PI;
        let f = 1.0 / (fov_y * 0.5).tan();
        let a = f / aspect;
        let b = f;
        let c = (z_far + z_near) / (z_near - z_far);
        let d = 2.0 * (z_far * z_near) / (z_near - z_far);

        Mat4x4([ a , 0.0,  0.0, 0.0,
                0.0,  b ,  0.0, 0.0,
                0.0, 0.0,   c ,-1.0,
                0.0, 0.0,   d , 0.0] )
    }

    pub fn row(&self, i : i32) -> Vec4 {
        match i {
            0 => Vec4([self.0[0], self.0[1], self.0[2], self.0[3]] ),
            1 => Vec4([self.0[4], self.0[5], self.0[6], self.0[7]] ),
            2 => Vec4([self.0[8], self.0[9], self.0[10],self.0[11]]),
            3 => Vec4([self.0[12],self.0[13],self.0[14],self.0[15]]),
            _ => panic!("row out of bounds")
        }
    }

    pub fn x(&self) -> Vec3 {
        self.row(0).xyz()
    }
    pub fn y(&self) -> Vec3 {
        self.row(1).xyz()
    }
    pub fn z(&self) -> Vec3 {
        self.row(2).xyz()
    }
    pub fn translation(&self) -> Vec3 {
        self.row(4).xyz()
    }

    pub fn transposed(&self) -> Mat4x4 {
        collect_mat( &mut (0..16).map(|i| {
            let (y,x) = (i/4,i%4);
            self.0[x*4+y]
            } ) )
    }

    pub fn col(&self, i : i32) -> Vec4 {
        match i {
            0 => Vec4([self.0[0+0], self.0[4+0], self.0[8+0], self.0[12+0]]),
            1 => Vec4([self.0[0+1], self.0[4+1], self.0[8+1], self.0[12+1]]),
            2 => Vec4([self.0[0+2], self.0[4+2], self.0[8+2], self.0[12+2]]),
            3 => Vec4([self.0[0+3], self.0[4+3], self.0[8+3], self.0[12+3]]),
            _ => panic!("col out of bounds")
        }
    }
}

pub fn collect_mat<I>(i : &mut I) -> Mat4x4 where I : Iterator<Item=f32> {
    Mat4x4([  i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(),
              i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(),
              i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(),
              i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap() ])
}



