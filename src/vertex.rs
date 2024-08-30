
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4]
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub fn generate_circle(aspect_ratio: f32, radius: f32, color: [f32; 4]) -> (Vec<Vertex>, Vec<u16>) {
    
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..360 {
        let radians = (i as f32).to_radians();
        let radians_next = (i as f32 + 1.0).to_radians();

        let x = radians.cos() * radius;
        let y = radians.sin() * radius * aspect_ratio;

        positions.push(Vertex {position: [x, y, 1.0], color});
        positions.push(Vertex {position: [radians_next.cos() * radius, radians_next.sin() * radius * aspect_ratio, 1.0], color});
        positions.push(Vertex {position: [0.0, 0.0, 1.0], color});

        if i == 0 {continue};
        indices.push(i);
        indices.push(0);
        indices.push(i + 1);
    };
    (positions, indices)
}
pub fn generate_ring(aspect_ratio: f32, radius: f32, inner_radius:f32, color: [f32; 4]) -> (Vec<Vertex>, Vec<u16>) {
    
    let mut positions = Vec::new();
    let mut indices = Vec::new();

   for i in 0..360 {
        let radians = (i as f32).to_radians();
        let radians_next = (i as f32 + 1.0).to_radians();

        let x = radians.cos() * radius;
        let y = radians.sin() * radius * aspect_ratio;

        let x_next = radians_next.cos() * radius;
        let y_next = radians_next.sin() * radius * aspect_ratio;

        positions.push(Vertex {position: [x, y, 1.0], color});
        positions.push(Vertex {position: [x_next, y_next, 1.0], color});
        positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color});

        positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color});
        positions.push(Vertex {position: [x_next, y_next, 1.0], color});
        positions.push(Vertex {position: [x_next * inner_radius, y_next * inner_radius, 1.0], color});

        if i == 0 {continue};
        indices.push(i);
        indices.push(0);
        indices.push(i + 1);
    };

    (positions, indices)
}

pub fn generate_glow_ring(aspect_ratio: f32, time:f32, outer_radius: f32, inner_radius:f32, color_first: [f32; 4], color_second: [f32; 4]) -> (Vec<Vertex>, Vec<u16>) {
    
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..360 {
        let radians = (i as f32).to_radians();
        let radians_next = (i as f32 + 1.0).to_radians();

        let time = time * 0.5;
        let x = radians.cos();
        let y = radians.sin() * aspect_ratio;

        let x_next = radians_next.cos();
        let y_next = radians_next.sin() * aspect_ratio;

        positions.push(Vertex {position: [x * (inner_radius + time), y * (inner_radius + time), 1.0], color: color_first});
        positions.push(Vertex {position: [x * (outer_radius + time), y * (outer_radius + time), 1.0], color: color_second});
        positions.push(Vertex {position: [x_next * (outer_radius + time), y_next * (outer_radius + time), 1.0], color: color_second});

        positions.push(Vertex {position: [x * (inner_radius + time), y * (inner_radius + time), 1.0], color: color_first});
        positions.push(Vertex {position: [x_next * (outer_radius + time), y_next * (outer_radius + time) , 1.0], color: color_second});
        positions.push(Vertex {position: [x_next * (inner_radius + time), y_next * (inner_radius + time), 1.0], color: color_first});
        
        if i == 0 {continue};
        indices.push(i);
        indices.push(0);
        indices.push(i + 1);
    };
    (positions, indices)
}