use crate::{
    heightfield::Heightfield, math::TriangleIndices as _, rasterize::RasterizationError,
    span::AreaType, trimesh::TriMesh,
};

impl TriMesh {
    /// Marks the triangles as walkable or not based on the threshold angle.
    ///
    /// The triangles are marked as walkable if the normal angle is greater than the threshold angle.
    ///
    /// # Arguments
    ///
    /// * `threshold_rad` - The threshold angle in radians.
    ///
    pub fn mark_walkable_triangles(&mut self, threshold_rad: f32) {
        let threshold_cos = threshold_rad.cos();
        for (i, indices) in self.indices.iter().enumerate() {
            let normal = indices.normal(&self.vertices);

            if normal.y > threshold_cos {
                self.area_types[i] = AreaType::DEFAULT_WALKABLE;
            }
        }
    }
}

impl Heightfield {
    /// Rasterizes the triangles of a [`TriMesh`] into a [`Heightfield`].
    ///
    /// # Arguments
    ///
    /// - `trimesh` - The [`TriMesh`] to rasterize.
    /// - `walkable_height` Minimum floor to 'ceiling' height that will still allow the floor area to be considered walkable. [Limit: >= 3] [Units: vx]
    /// - `walkable_climb` - Minimum floor to 'ceiling' height that will still allow the floor area to be considered walkable. [Limit: >= 3] [Units: vx]
    ///
    pub fn populate_from_trimesh(
        &mut self,
        trimesh: TriMesh,
        walkable_height: u16,
        walkable_climb: u16,
    ) -> Result<(), RasterizationError> {
        // Implementation note: flag_merge_threshold and walkable_climb_height are the same thing in practice, so we just chose one name for the param.

        // Find triangles which are walkable based on their slope and rasterize them.
        for (i, triangle) in trimesh.indices.iter().enumerate() {
            let triangle = [
                trimesh.vertices[triangle[0] as usize],
                trimesh.vertices[triangle[1] as usize],
                trimesh.vertices[triangle[2] as usize],
            ];
            let area_type = trimesh.area_types[i];
            self.rasterize_triangle(triangle, area_type, walkable_climb)?;
        }
        // Once all geometry is rasterized, we do initial pass of filtering to
        // remove unwanted overhangs caused by the conservative rasterization
        // as well as filter spans where the character cannot possibly stand.
        self.filter_low_hanging_walkable_obstacles(walkable_climb);
        self.filter_ledge_spans(walkable_height, walkable_climb);
        self.filter_walkable_low_height_spans(walkable_height);
        Ok(())
    }
}
