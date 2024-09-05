use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use wgpu::BlendComponent;
use winit::window::Window;

use crate::{buffers, setup::{self, Preload}, vertex_generator::Vertex};

pub struct State<'a> {
    window: &'a Window,
    hardware: Preload<'a>,
    render_pipeline: wgpu::RenderPipeline,
    index_buffers: Vec<(wgpu::Buffer, u32)>,
    is_record: Arc<Mutex<bool>>,
    time:f32,
    is_inc: bool
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
        let time = 0.0;
        let index_buffers = buffers::create_index(&hardware.device);
        Self {
            window,
            hardware,
            render_pipeline,
            index_buffers,
            time,
            is_record: Arc::new(Mutex::new(false)),
            is_inc: true,
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
    pub fn start_record(&self) {
        let pointer = Arc::clone(&self.is_record);
        thread::spawn(move || {
            println!("Start recording");
            *pointer.lock().unwrap() = true;
            sleep(Duration::new(5, 0));
            *pointer.lock().unwrap() = false;
            println!("Stop recording");
        });
    }
    pub fn render(&mut self) {

        if *self.is_record.lock().unwrap() == true {
            //print!("RECORD-");
        }
        //update
        if self.time < -0.03 {
            self.is_inc = true;
        }
        else if self.time > 0.03 {
            self.is_inc = false;
        }
        if self.is_inc {
            self.time += 0.00005;
        }
        else {
            self.time -= 0.00005;
        }
  
        let aspect_ratio = self.hardware.size.width as f32 / self.hardware.size.height as f32;

        let vertex_buffers = buffers::create_vertex(aspect_ratio, &self.hardware.device, self.time);
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
        // Передаем буфер в очередь команд устройства
        self.hardware.queue.submit(Some(encoder.finish()));
        // Отображаем готовый кадр
        frame.present();
    }
}
