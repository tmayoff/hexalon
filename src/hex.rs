use std::ops;

use bevy_egui::egui::lerp;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

pub struct FractionalHexCoord {
    pub q: f32,
    pub r: f32,
}

impl HexCoord {
    pub fn lerp(a: &HexCoord, b: &HexCoord, t: f32) -> HexCoord {
        Self::round(FractionalHexCoord {
            q: lerp(a.q as f32..=b.q as f32, t),
            r: lerp(a.r as f32..=b.r as f32, t),
        })
    }

    pub fn round(coord: FractionalHexCoord) -> HexCoord {
        let mut q = (coord.q).round();
        let mut r = (coord.r).round();
        let s = (-coord.q - coord.r).round();

        let q_diff = (q - coord.q).abs();
        let r_diff = (r - coord.r).abs();
        let s_diff = (s - (-coord.q - coord.r)).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s
        } else {
            // s = -q - r
        }

        HexCoord {
            q: q as i32,
            r: r as i32,
        }
    }

    pub fn distance(&self, other: &HexCoord) -> i32 {
        let vec = self - other;
        (vec.q.abs() + vec.r.abs() + (vec.q + vec.r).abs()) / 2
    }
}

impl ops::Sub<&HexCoord> for &HexCoord {
    type Output = HexCoord;

    fn sub(self, rhs: &HexCoord) -> Self::Output {
        HexCoord {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}
