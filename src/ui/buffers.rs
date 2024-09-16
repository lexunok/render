use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::ui::colors::*;
use crate::ui::index_generator;
use crate::ui::vertex_generator::{self, Vertex};

pub fn create_vertex(aspect_ratio: f32, device: &Device, time:f32) ->  Vec<Buffer> {
    let mut dynamic_color = [0.462745098 - time * 15.5 , 0.6 - time * 20.0, 1.0 - time * 34.0, 0.5];
    if time > 0.0295 {
        dynamic_color = [0.0, 0.0, 0.0, 0.5];
    }

    let ring = vertex_generator::generate_ring(aspect_ratio, 0.43, 0.34, PURPLE);
    let glow_ring_outer = vertex_generator::generate_glow_ring(aspect_ratio, time, 0.45,0.4, PURPLE, BLACK);
    let glow_ring_inner = vertex_generator::generate_glow_ring(aspect_ratio, time, 0.4,0.3, BLACK, dynamic_color);
    let circle = vertex_generator::generate_circle(aspect_ratio, 0.5, PURPLE);
    
    vec![
        get_vertex_buffer(ring, device),
        get_vertex_buffer(glow_ring_outer, device),
        get_vertex_buffer(glow_ring_inner, device),
        get_vertex_buffer(circle, device),
        ]
}
pub fn create_index(device: &Device) -> Vec<(Buffer, u32)> {
    let ring = index_generator::generate_ring();
    let circle = index_generator::generate_circle();
    let partial_ring = index_generator::generate_partial_ring();

    let ring_len = ring.len() as u32;
    let circle_len = circle.len() as u32;
    let partial_ring_len = partial_ring.len() as u32;

    vec![
        (get_index_buffer(ring, &device), ring_len),
        (get_index_buffer(circle, &device), circle_len),
        (get_index_buffer(partial_ring, &device), partial_ring_len),
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