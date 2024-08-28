use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::vertex::{self, Vertex};


const BLACK:[f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE:[f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PURPLE:[f32; 4] = [0.462745098, 0.584313725, 1.0, 1.0];

pub fn create(aspect_ratio: f32, device: &Device) ->  (Vec<Buffer>, Vec<Buffer>) {
    let mut verices = Vec::new();
    let mut indices = Vec::new();

    let ring = vertex::generate_ring(aspect_ratio, 0.4, 0.8, PURPLE);
    let glow_ring_outer = vertex::generate_glow_ring(aspect_ratio, 0.45,0.8, PURPLE, BLACK);
    let glow_ring_inner = vertex::generate_glow_ring(aspect_ratio, 0.35,0.8, BLACK, PURPLE);

    let ring = get_index_vertex_buffer(ring, device);
    let glow_ring_outer = get_index_vertex_buffer(glow_ring_outer, device);
    let glow_ring_inner = get_index_vertex_buffer(glow_ring_inner, device);

    verices.push(ring);
    verices.push(glow_ring_outer);
    verices.push(glow_ring_inner);

    // indices.push(res_1.1);
    // indices.push(res_2.1);
    // indices.push(res_3.1);

    (verices, indices)
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