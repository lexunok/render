use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::index_generator;
use crate::vertex_generator::{self, Vertex};

//todo
const BLACK:[f32; 4] = [0.0, 0.0, 0.0, 1.0];
//const WHITE:[f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PURPLE:[f32; 4] = [0.462745098, 0.584313725, 1.0, 1.0];

pub fn create_vertex(aspect_ratio: f32, device: &Device, time:f32) ->  Vec<Buffer> {

    let dynamic_color = [0.462745098 + (time * 4.0), 0.6 + (time * 4.0), 1.0, 0.5];

    let ring = vertex_generator::generate_ring(aspect_ratio, 0.41, 0.34, PURPLE);
    let glow_ring_outer = vertex_generator::generate_glow_ring(aspect_ratio, time, 0.45,0.4, dynamic_color, BLACK);
    let glow_ring_inner = vertex_generator::generate_glow_ring(aspect_ratio, time, 0.4,0.3, BLACK, dynamic_color);
    
    vec![
        get_vertex_buffer(ring, device),
        get_vertex_buffer(glow_ring_outer, device),
        get_vertex_buffer(glow_ring_inner, device)
        ]
}
pub fn create_index(device: &Device) -> Vec<(Buffer, u32)> {
    let ring = index_generator::generate_ring();
    let ring_len = ring.len() as u32;

    vec![
        (get_index_buffer(ring, &device), ring_len)
        ]
}

fn get_vertex_buffer(shape: Vec<Vertex>, device: &Device) -> wgpu::Buffer {
    //Создаем вертекс буфер
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&shape),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    vertex_buffer
}
fn get_index_buffer(indices: Vec<u16>, device: &Device) -> wgpu::Buffer {
    //Индекс буфер
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    index_buffer
}