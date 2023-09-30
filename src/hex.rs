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
        let qgrid = coord.q.round() as i32;
        let rgrid = coord.r.round() as i32;

        let q = coord.q - qgrid as f32;
        let r = coord.r - rgrid as f32;

        if q.abs() >= r.abs() {
            HexCoord {
                q: qgrid + (q + 0.5 * r).round() as i32,
                r: rgrid,
            }
        } else {
            HexCoord {
                q: qgrid,
                r: rgrid + (r + 0.5 * q).round() as i32,
            }
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

impl ops::Add<HexCoord> for HexCoord {
    type Output = HexCoord;

    fn add(self, rhs: HexCoord) -> Self::Output {
        HexCoord {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl ops::Add<&HexCoord> for &HexCoord {
    type Output = HexCoord;

    fn add(self, rhs: &HexCoord) -> Self::Output {
        HexCoord {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}
