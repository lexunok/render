use winit::{event::WindowEvent, window::Window};
use wgpu::util::DeviceExt;

use crate::vertex::{generate_circle, Vertex};

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    num_indices: u32
}

impl<'a> State<'a> {

    pub async fn new(window: &'a Window) -> State<'a> {
        //Устанавливаем размер окна
        let size = window.inner_size();
    
        let instance = wgpu::Instance::default();
        //Создаем поверхность
        let surface = instance.create_surface(window).unwrap();
        //Запрашиваем адаптер для работы с графикой
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await.unwrap();
        //Создаем устройство из адаптера
        let (device, queue) = adapter
            .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await.unwrap();

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(swapchain_capabilities.formats[0]);
        //Создаем конфиг для поверхности
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: swapchain_capabilities.present_modes[0],
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        //Создаем объект шейдера
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    
        //Создаем графический конвейер
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    format: config.format,
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
        let circle = generate_circle();
        //Создаем вертекс буфер
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&circle.0),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        //Индекс буфер
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&circle.1),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = circle.1.len() as u32;
        
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    //Метод - при изменении размера окна нужно переконфигурировать размер поверхности
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        //todo!()
    }

    pub fn render(&mut self) {
        // Получаем следующий кадр.
        let frame = self.surface.get_current_texture().unwrap();
        // Создаём View для изображения этого кадра.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Создаем объект для записи последовательности команд рендеринга в буфер для его передачи в устройство на выполнение
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: None});
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
        }
        // Передаем буфер в очередь команд устройства
        self.queue.submit(Some(encoder.finish()));
        // Отображаем готовый кадр
        frame.present();
    }
}
