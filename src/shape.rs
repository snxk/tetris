use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    position: HashSet<Pos>,
    anchor: Pos,
}

macro_rules! impl_shape_constructor {
    ($($new:ident: [$($pos:expr), *] anchored at $anchor:expr;)*) => {
      $(
        pub fn $new() -> Self {
          Self {
            position: [$ ($pos), *].iter().cloned().collect(),
            anchor: $anchor,
          }
        }
      )*
    };
}

impl Shape {
    impl_shape_constructor! {
      new_i: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] anchored at  Pos(1, 0);
      new_o: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] anchored at  Pos(0, 0);
      new_t: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] anchored at  Pos(1, 0);
      new_j: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] anchored at  Pos(0, 1);
      new_l: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] anchored at  Pos(0, 1);
      new_s: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] anchored at  Pos(0, 0);
      new_z: [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] anchored at  Pos(0, 0);
    }

    pub fn new_random() -> Self {
        let random = (rand::random::<f64>() * 7.0) as u8;

        match random {
            0 => Self::new_i(),
            1 => Self::new_o(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }
}
