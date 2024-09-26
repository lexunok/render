use bytemuck::NoUninit;
use cgmath::Rad;
use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::ui::colors::*;
use crate::ui::index_generator;
use crate::ui::vertex_generator::{self, Vertex};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformUniform {
    value: [[f32; 4]; 4],
}

pub fn create_vertex(device: &Device) ->  Vec<Buffer> {

    let glow_ring_outer = vertex_generator::generate_glow_ring(0.45,0.4, PURPLE, BLACK);
    let glow_ring_inner = vertex_generator::generate_glow_ring(0.4,0.3, BLACK, PURPLE);
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
pub fn create_uniform(aspect_ratio:f32, scale:f32, rotation: f32 , device: &Device) -> Vec<Buffer> {
    let aspect_ratio_uniform = [1.0, aspect_ratio, 1.0];

    let transform = cgmath::Matrix4::from_scale(scale) * cgmath::Matrix4::from_axis_angle(cgmath::Vector3 { x: 0.0, y: 0.0, z: -1.0 }, Rad(rotation)) * OPENGL_TO_WGPU_MATRIX;
    let transform_uniform = TransformUniform {value: transform.into()};

    vec![
        get_uniform_buffer(aspect_ratio_uniform, &device),
        get_uniform_buffer(transform_uniform, &device)
        ]
}

///////////////////////////////////
// СОЗДАНИЕ БУФФЕРОВ В УСТРОЙСТВЕ//
///////////////////////////////////
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
fn get_uniform_buffer<T: NoUninit>(uniform: T, device: &Device) -> wgpu::Buffer {
    let uniform_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );
    uniform_buffer
}