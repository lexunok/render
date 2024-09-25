use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::ui::colors::*;
use crate::ui::index_generator;
use crate::ui::vertex_generator::{self, Vertex};

pub fn create_vertex(device: &Device, time:f32) ->  Vec<Buffer> {

    let glow_ring_outer = vertex_generator::generate_glow_ring(time, 0.45,0.4, PURPLE, BLACK);
    let glow_ring_inner = vertex_generator::generate_glow_ring(time, 0.4,0.3, BLACK, PURPLE);
    let triangle = vertex_generator::generate_triangle();
    
    vec![
        get_vertex_buffer(glow_ring_outer, device),
        get_vertex_buffer(glow_ring_inner, device),
        get_vertex_buffer(triangle, device),
        ]
}
pub fn create_index(device: &Device) -> Vec<(Buffer, u32)> {
    let ring = index_generator::generate_ring();

    let ring_len = ring.len() as u32;

    vec![
        (get_index_buffer(ring, &device), ring_len),
        ]
}
pub fn create_uniform(aspect_ratio:f32, device: &Device) -> Vec<Buffer> {
    vec![
        get_uniform_buffer(vec![1.0, aspect_ratio, 1.0], &device),
        ]
}

fn get_vertex_buffer(shape: Vec<Vertex>, device: &Device) -> wgpu::Buffer {
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
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    index_buffer
}
fn get_uniform_buffer(uniform: Vec<f32>, device: &Device) -> wgpu::Buffer {
    let uniform_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );
    uniform_buffer
}