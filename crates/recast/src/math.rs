use glam::{UVec3, Vec2, Vec3A};

/// A 3D axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Aabb3d {
    /// The minimum point of the box
    pub min: Vec3A,
    /// The maximum point of the box
    pub max: Vec3A,
}

impl Aabb3d {
    /// Constructs an AABB from its center and half-size.
    #[inline]
    pub fn new(center: impl Into<Vec3A>, half_size: impl Into<Vec3A>) -> Self {
        let (center, half_size) = (center.into(), half_size.into());
        debug_assert!(half_size.x >= 0.0 && half_size.y >= 0.0 && half_size.z >= 0.0);
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    /// Constructs an AABB from a list of vertices.
    ///
    /// Returns `None` if the list of vertices is empty.
    #[inline]
    pub(crate) fn from_verts(verts: &[Vec3A]) -> Option<Self> {
        if verts.is_empty() {
            return None;
        }
        let min = verts
            .iter()
            .fold(Vec3A::splat(f32::MAX), |acc, &v| acc.min(v));
        let max = verts
            .iter()
            .fold(Vec3A::splat(f32::MIN), |acc, &v| acc.max(v));
        Some(Self { min, max })
    }

    /// Checks if this AABB intersects with another AABB.
    #[inline]
    pub(crate) fn intersects(&self, other: &Aabb3d) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }
}

/// A 2D axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb2d {
    /// The minimum point of the box
    pub min: Vec2,
    /// The maximum point of the box
    pub max: Vec2,
}

impl Aabb2d {
    /// Creates a new AABB from a list of 2D points.
    ///
    /// Returns `None` if the list of vertices is empty.
    #[inline]
    pub(crate) fn from_verts(verts: &[Vec2]) -> Option<Self> {
        if verts.is_empty() {
            return None;
        }
        let min = verts
            .iter()
            .fold(Vec2::splat(f32::MAX), |acc, &v| acc.min(v));
        let max = verts
            .iter()
            .fold(Vec2::splat(f32::MIN), |acc, &v| acc.max(v));
        Some(Self { min, max })
    }

    /// Extends the AABB into an [`Aabb3d`] by treating the existing coordinates as X and Z values,
    /// and `y_min` and `y_max` are the new minimum and maximum Y values.
    #[inline]
    pub(crate) fn extend_y(self, y_min: f32, y_max: f32) -> Aabb3d {
        Aabb3d {
            min: Vec3A::new(self.min.x, y_min, self.min.y),
            max: Vec3A::new(self.max.x, y_max, self.max.y),
        }
    }
}

pub(crate) trait TriangleIndices {
    fn normal(&self, vertices: &[Vec3A]) -> Vec3A;
}

impl TriangleIndices for UVec3 {
    #[inline]
    fn normal(&self, vertices: &[Vec3A]) -> Vec3A {
        let a = vertices[self[0] as usize];
        let b = vertices[self[1] as usize];
        let c = vertices[self[2] as usize];
        let ab = b - a;
        let ac = c - a;
        ab.cross(ac).normalize_or_zero()
    }
}

pub(crate) trait TriangleVertices {
    fn aabb(&self) -> Aabb3d;
}

impl TriangleVertices for [Vec3A; 3] {
    #[inline]
    fn aabb(&self) -> Aabb3d {
        let min = self[0].min(self[1]).min(self[2]);
        let max = self[0].max(self[1]).max(self[2]);
        Aabb3d { min, max }
    }
}

/// Gets the standard width (x-axis) offset for the specified direction.
/// # Arguments
/// - `direction`: The direction. [Limits: 0 <= value < 4]
/// # Returns
///
/// The width offset to apply to the current cell position to move in the direction.
#[inline]
pub(crate) fn dir_offset_x(direction: u8) -> i8 {
    const OFFSET: [i8; 4] = [-1, 0, 1, 0];
    OFFSET[direction as usize & 0x03]
}

/// Gets the standard height (z-axis) offset for the specified direction.
/// # Arguments
/// - `direction`: The direction. [Limits: 0 <= value < 4]
/// # Returns
///
/// The height offset to apply to the current cell position to move in the direction.
#[inline]
pub(crate) fn dir_offset_z(direction: u8) -> i8 {
    const OFFSET: [i8; 4] = [0, 1, 0, -1];
    OFFSET[direction as usize & 0x03]
}
