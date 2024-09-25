
use wgpu::{BindGroupLayout, BlendComponent};
use winit::window::Window;

use crate::ui::{buffers, setup::{self, Preload}, vertex_generator::Vertex};

pub struct State<'a> {
    window: &'a Window,
    hardware: Preload<'a>,
    render_pipeline: wgpu::RenderPipeline,
    index_buffers: Vec<(wgpu::Buffer, u32)>,
    uniform_bind_group_layout: BindGroupLayout,
    is_record: bool,
    counter:i16,
    direction: i16
}

impl<'a> State<'a> {

    pub async fn new(window: &'a Window) -> State<'a> {
        // Настройка поверхности и устройства
        let hardware = setup::start(window).await;
        
        //Создаем объект шейдера
        let shader = hardware.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));   

        let uniform_bind_group_layout = hardware.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: None,
        });

        let pipeline_layout = hardware.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
    
        //Создаем графический конвейер
        let render_pipeline = hardware.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ],
                compilation_options:  wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options:  wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: hardware.config.format,
                    blend: Some(wgpu::BlendState {
                        color: BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        },
                        alpha: BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        }
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, 
            multisample: wgpu::MultisampleState {
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false, 
            },
            multiview: None,
            cache: None, 
        });
        let index_buffers = buffers::create_index(&hardware.device);

        Self {
            window,
            hardware,
            render_pipeline,
            index_buffers,
            uniform_bind_group_layout,
            is_record: false,
            counter: 0,
            direction: 1,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    //Метод - при изменении размера окна нужно переконфигурировать размер поверхности
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.hardware.size = new_size;
        self.hardware.config.width = new_size.width;
        self.hardware.config.height = new_size.height;
        self.hardware.surface.configure(&self.hardware.device, &self.hardware.config);
    }
    pub fn start_record(&mut self) {
        self.is_record = !self.is_record;
        self.counter = 0;
        self.direction = 1;
        if self.is_record {
            println!("Start recording");
            //stream play
        }
        else {
            println!("Stop recording");
            //stream drop
        }
    }
    pub fn render(&mut self) {

        self.counter += self.direction;
    
        if self.counter == 0 || (self.counter == 300 && !self.is_record) || (self.counter == 100 && self.is_record){
            self.direction = -self.direction
        }

        let time = self.counter as f32;
        let aspect_ratio = self.hardware.size.width as f32 / self.hardware.size.height as f32;

        let vertex_buffers = buffers::create_vertex(&self.hardware.device, time / 10000.0);
        let uniform_buffers = buffers::create_uniform(aspect_ratio, &self.hardware.device);

        let uniform_bind_group = &self.hardware.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffers[0].as_entire_binding(),
                }
            ],
            label: None,
        }); 

        let frame = self.hardware.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.hardware.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: None});
        {
            let mut rpass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store, 
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, uniform_bind_group, &[]);

            if !self.is_record {
                rpass.set_vertex_buffer(0, vertex_buffers[0].slice(..));
                rpass.set_index_buffer(self.index_buffers[0].0.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..self.index_buffers[0].1, 0, 0..1);
    
                rpass.set_vertex_buffer(0, vertex_buffers[1].slice(..));
                rpass.set_index_buffer(self.index_buffers[0].0.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..self.index_buffers[0].1,0, 0..1);
    
                rpass.set_vertex_buffer(0, vertex_buffers[2].slice(..));
                rpass.set_index_buffer(self.index_buffers[0].0.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..self.index_buffers[0].1,0, 0..1);   
            }
            else if self.is_record{

            }
        }
        self.hardware.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
