use winit::window::Window;

pub struct Preload<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

pub async fn start(window: &Window) -> Preload {
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
            let swapchain_format = swapchain_capabilities.formats[0];
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

            Preload {
                surface,
                device,
                queue,
                config,
                size
            }
}