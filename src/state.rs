use winit::{event::WindowEvent, window::Window};
use wgpu::util::DeviceExt;

use crate::{setup::{self, Preload}, vertex::{generate_circle, Vertex}};

const BLACK:[f32; 3] = [0.0, 0.0, 0.0];
const PURPLE:[f32; 3] = [0.462745098, 0.584313725, 1.0];

pub struct State<'a> {
    window: &'a Window,
    hardware: Preload<'a>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_buffer_2: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_buffer_2: wgpu::Buffer,
    num_indices: u32
}

impl<'a> State<'a> {

    pub async fn new(window: &'a Window) -> State<'a> {
        // Настройка поверхности и устройства
        let hardware = setup::start(window).await;
        
        //Создаем объект шейдера
        let shader = hardware.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));    

        let pipeline_layout = hardware.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    
        //Создаем графический конвейер
        let render_pipeline = hardware.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            //Указываем вертекс шейдеры (позиции объекта)
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ],
                compilation_options:  wgpu::PipelineCompilationOptions::default(),
            },
            //Указываем фрагмент шейдеры (раскрашивание пикселей)
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options:  wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: hardware.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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

        let aspect_ratio = hardware.size.width as f32 / hardware.size.height as f32;
        let circle = generate_circle(aspect_ratio, 0.4, PURPLE);
        let circle_2 = generate_circle(aspect_ratio, 0.3, BLACK);
        //Создаем вертекс буфер
        let vertex_buffer = hardware.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&circle.0),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let vertex_buffer_2 = hardware.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&circle_2.0),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        //Индекс буфер
        let index_buffer = hardware.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&circle.1),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let index_buffer_2 = hardware.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&circle_2.1),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = circle.1.len() as u32;
        
        Self {
            window,
            hardware,
            render_pipeline,
            vertex_buffer,
            vertex_buffer_2,
            index_buffer,
            index_buffer_2,
            num_indices
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

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        //todo!()
    }

    pub fn render(&mut self) {
        // Получаем следующий кадр.
        let frame = self.hardware.surface.get_current_texture().unwrap();
        // Создаём View для изображения этого кадра.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Создаем объект для записи последовательности команд рендеринга в буфер для его передачи в устройство на выполнение
        let mut encoder = self.hardware.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: None});
        //Новая область видимости чтобы RenderPass жил не дольше чем CommandEncoder
        {
            let mut rpass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view, // Цель для отрисовки
                        resolve_target: None, // Используется для мультисэмплинга
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),  // Очищаем кадр черным цветом
                            store: wgpu::StoreOp::Store, // Сохраняем содержимое после завершения данного RenderPass
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            //Задаем графический конвейер
            rpass.set_pipeline(&self.render_pipeline);
            //Устанавливаем буферы
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            //Рисуем объекты с вершинами и кол-вом
            rpass.draw_indexed(0..self.num_indices,0, 0..1);

            rpass.set_vertex_buffer(0, self.vertex_buffer_2.slice(..));
            rpass.set_index_buffer(self.index_buffer_2.slice(..), wgpu::IndexFormat::Uint16);
            rpass.draw_indexed(0..self.num_indices,0, 0..1);
        }
        // Передаем буфер в очередь команд устройства
        self.hardware.queue.submit(Some(encoder.finish()));
        // Отображаем готовый кадр
        frame.present();
    }
}
