
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub fn generate_circle(aspect_ratio: f32) -> (Vec<Vertex>, Vec<u16>) {
    
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    //Центр круга
    positions.push(Vertex {position: [0.0, 0.0, 0.0], color: [0.0,0.0,0.0]});

    for i in 0..361 {
        let radians = (i as f32).to_radians();
        let x = radians.sin() * 0.5;
        let y = radians.cos() * 0.5 * aspect_ratio;

        positions.push(Vertex {position: [x, y, 0.0], color: [0.5,0.2,0.5]});
        if i == 0 {continue};
        indices.push(i);
        indices.push(0);
        indices.push(i + 1);
    };
    (positions, indices)
}

