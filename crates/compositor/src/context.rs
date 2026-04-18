use crate::error::CompositorError;

/// GPU context holding wgpu instance, adapter, device, and queue.
/// This is the foundation for all GPU-accelerated compositing.
pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter_info: wgpu::AdapterInfo,
}

impl GpuContext {
    /// Initialize the GPU context. Requires a compatible GPU adapter.
    pub async fn new() -> Result<Self, CompositorError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(CompositorError::NoAdapter)?;

        let adapter_info = adapter.get_info();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(|e| CompositorError::DeviceRequest(e.to_string()))?;

        Ok(Self {
            device,
            queue,
            adapter_info,
        })
    }
}

impl std::fmt::Debug for GpuContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuContext")
            .field("adapter", &self.adapter_info.name)
            .field("backend", &self.adapter_info.backend)
            .finish()
    }
}
