
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
pub fn generate_circle(aspect_ratio: f32, radius: f32, color: [f32; 4]) -> Vec<Vertex> {
    
    let mut positions = Vec::new();

    let start_radian = (0 as f32).to_radians();
    let start_x = start_radian.sin() * radius;
    let start_y = start_radian.cos() * radius * aspect_ratio;

    positions.push(Vertex {position: [0.0, 0.0, 1.0], color});
    positions.push(Vertex {position: [start_x, start_y, 1.0], color});

    for i in 1..361 {
        let radians = (i as f32).to_radians();

        let x = radians.sin() * radius;
        let y = radians.cos() * radius * aspect_ratio;

        positions.push(Vertex {position: [x, y, 1.0], color});
    };
    positions
}
//todo совместить generate ring и generate glow ring
pub fn generate_ring(aspect_ratio: f32, outer_radius: f32, inner_radius:f32, color: [f32; 4]) -> Vec<Vertex> {
    
    let mut positions = Vec::new();

    for i in (0..360).step_by(2) {
        let radians = (i as f32 + 1.0).to_radians();

        let x = radians.cos();
        let y = radians.sin() * aspect_ratio;

        if i == 0 {
            let radians = (i as f32).to_radians();
            let x = radians.cos();
            let y = radians.sin() * aspect_ratio;
            positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color}); 
            positions.push(Vertex {position: [x * outer_radius, y * outer_radius, 1.0], color}); 
        }
        positions.push(Vertex {position: [x * outer_radius, y * outer_radius, 1.0], color}); 
        positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color}); 

    };
    let x = (360 as f32).to_radians().cos();
    let y = (360 as f32).to_radians().sin() * aspect_ratio;
    positions.push(Vertex {position: [x * outer_radius, y *  outer_radius, 1.0], color});
    positions.push(Vertex {position: [x * inner_radius, y * inner_radius , 1.0], color});

    positions
}

pub fn generate_glow_ring(aspect_ratio: f32, time:f32, outer_radius: f32, inner_radius:f32, color_first: [f32; 4], color_second: [f32; 4]) -> Vec<Vertex> {
    
    let mut positions = Vec::new();

    for i in (0..360).step_by(2) {
        let radians = (i as f32 + 1.0).to_radians();

        let x = radians.cos();
        let y = radians.sin() * aspect_ratio;

        if i == 0 {
            let radians = (i as f32).to_radians();
            let x = radians.cos();
            let y = radians.sin() * aspect_ratio;
            positions.push(Vertex {position: [x * (inner_radius + time), y * (inner_radius + time), 1.0], color: color_first}); 
            positions.push(Vertex {position: [x * (outer_radius + time), y * (outer_radius + time), 1.0], color: color_second}); 
        }
        positions.push(Vertex {position: [x * (outer_radius + time), y * (outer_radius + time), 1.0], color: color_second}); 
        positions.push(Vertex {position: [x * (inner_radius + time), y * (inner_radius + time), 1.0], color: color_first}); 
    };
    let x = (360 as f32).to_radians().cos();
    let y = (360 as f32).to_radians().sin() * aspect_ratio;
    positions.push(Vertex {position: [x * (outer_radius + time), y *  (outer_radius + time), 1.0], color: color_second});
    positions.push(Vertex {position: [x * (inner_radius + time), y * (inner_radius + time), 1.0], color: color_first});

    positions
}
pub fn generate_static_glow_ring(aspect_ratio: f32, outer_radius: f32, inner_radius:f32, color_first: [f32; 4], color_second: [f32; 4]) -> Vec<Vertex> {
    
    let mut positions = Vec::new();

    for i in (0..360).step_by(2) {
        let radians = (i as f32 + 1.0).to_radians();

        let x = radians.cos();
        let y = radians.sin() * aspect_ratio;

        if i == 0 {
            let radians = (i as f32).to_radians();
            let x = radians.cos();
            let y = radians.sin() * aspect_ratio;
            positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color: color_first}); 
            positions.push(Vertex {position: [x * outer_radius, y * outer_radius, 1.0], color: color_second}); 
        }
        positions.push(Vertex {position: [x * outer_radius, y * outer_radius, 1.0], color: color_second}); 
        positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color: color_first}); 
    };
    let x = (360 as f32).to_radians().cos();
    let y = (360 as f32).to_radians().sin() * aspect_ratio;
    positions.push(Vertex {position: [x * outer_radius, y *  outer_radius, 1.0], color: color_second});
    positions.push(Vertex {position: [x * inner_radius, y * inner_radius, 1.0], color: color_first});

    positions
}