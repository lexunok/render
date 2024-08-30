use wgpu::{Buffer, Color, Device};
use wgpu::util::DeviceExt;
use crate::vertex::{self, Vertex};


const BLACK:[f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE:[f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PURPLE:[f32; 4] = [0.462745098, 0.584313725, 1.0, 1.0];

pub fn create(aspect_ratio: f32, device: &Device, time:f32) ->  Vec<Buffer> {
    let mut verices = Vec::new();
   
    let color = [0.462745098+ (time * 4.0), 0.6 + (time * 4.0), 1.0, 0.5];
    let ring = vertex::generate_ring(aspect_ratio, 0.4, 0.85, PURPLE);
    let glow_ring_outer = vertex::generate_glow_ring(aspect_ratio, time, 0.45,0.4, color, BLACK);
    let glow_ring_inner = vertex::generate_glow_ring(aspect_ratio, time, 0.4,0.3, BLACK, color);

    let ring = get_index_vertex_buffer(ring, device);
    let glow_ring_outer = get_index_vertex_buffer(glow_ring_outer, device);
    let glow_ring_inner = get_index_vertex_buffer(glow_ring_inner, device);

    verices.push(ring);
    verices.push(glow_ring_outer);
    verices.push(glow_ring_inner);

    // indices.push(res_1.1);
    // indices.push(res_2.1);
    // indices.push(res_3.1);

    verices
}

fn get_index_vertex_buffer(shape: (Vec<Vertex>, Vec<u16>), device: &Device) -> wgpu::Buffer {
    //Создаем вертекс буфер
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&shape.0),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    //Индекс буфер
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&shape.1),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    vertex_buffer
}