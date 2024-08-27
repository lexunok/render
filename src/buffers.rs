use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::vertex::{self, Vertex};

const BLACK:[f32; 3] = [0.0, 0.0, 0.0];
const PURPLE:[f32; 3] = [0.462745098, 0.584313725, 1.0];

pub fn create(aspect_ratio: f32, device: &Device) ->  (Vec<Buffer>, Vec<Buffer>) {
    let mut verices = Vec::new();
    let mut indices = Vec::new();

    let circle_1 = vertex::generate_circle(aspect_ratio, 0.4, PURPLE);
    let circle_2 = vertex::generate_circle(aspect_ratio, 0.3, BLACK);

    let res_1 = get_index_vertex_buffer(circle_1, device);
    let res_2 = get_index_vertex_buffer(circle_2, device);

    verices.push(res_1.0);
    verices.push(res_2.0);

    indices.push(res_1.1);
    indices.push(res_2.1);

    (verices, indices)
}

fn get_index_vertex_buffer(shape: (Vec<Vertex>, Vec<u16>), device: &Device) -> (wgpu::Buffer, wgpu::Buffer) {
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
    (vertex_buffer, index_buffer)
}