use utils::log;

pub struct MeshData {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uv0: Vec<f32>,
    indices: Vec<u32>,
    raw_vertices: Vec<f32>,
    raw_indices: Vec<u32>,
}

impl MeshData {
    pub fn new(positions: Vec<f32>, normals: Vec<f32>, uv0: Vec<f32>, indices: Vec<u32>) -> Self {
        const VERTEX_DATA_SIZE: usize = 3 + 3 + 2;

        let mut raw_vertices = vec![0f32; indices.len()*VERTEX_DATA_SIZE];
        let mut raw_indices = vec![0u32; indices.len()];

        for (i, &v) in indices.iter().enumerate() {
            let index = v as usize;

            raw_vertices[i * VERTEX_DATA_SIZE + 0] = positions[index * 3 + 0];
            raw_vertices[i * VERTEX_DATA_SIZE + 1] = positions[index * 3 + 1];
            raw_vertices[i * VERTEX_DATA_SIZE + 2] = positions[index * 3 + 2];

            raw_vertices[i * VERTEX_DATA_SIZE + 3] = normals[i * 3 + 0];
            raw_vertices[i * VERTEX_DATA_SIZE + 4] = normals[i * 3 + 1];
            raw_vertices[i * VERTEX_DATA_SIZE + 5] = normals[i * 3 + 2];
            
            raw_vertices[i * VERTEX_DATA_SIZE + 6] = uv0[i * 2 + 0];
            raw_vertices[i * VERTEX_DATA_SIZE + 7] = uv0[i * 2 + 1];

            raw_indices[i] = i as u32;
        }

        MeshData {
            positions,
            normals,
            uv0,
            indices: indices,

            raw_vertices,
            raw_indices,
        }
    }

    pub fn raw_vertices (&self) -> &[f32] {
        &self.raw_vertices
    }

    pub fn raw_indices (&self) -> &[u32] {
        &self.raw_indices
    }
}