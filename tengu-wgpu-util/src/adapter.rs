use crate::{device::DeviceBuilder, Error, Surface};

pub struct Adapter(wgpu::Adapter);

impl Adapter {
    pub fn device(self) -> DeviceBuilder {
        DeviceBuilder::new(self.0)
    }
}

// Adapter builder implementation.

pub struct AdapterBuilder<'surface, 'window> {
    instance: wgpu::Instance,
    request_adapter_options: wgpu::RequestAdapterOptions<'surface, 'window>,
}

impl<'surface, 'window> AdapterBuilder<'surface, 'window> {
    pub fn new(instance: wgpu::Instance) -> Self {
        Self {
            instance,
            request_adapter_options: wgpu::RequestAdapterOptions::default(),
        }
    }

    pub fn with_surface(mut self, surface: &'surface Surface<'window>) -> Self {
        self.request_adapter_options.compatible_surface = Some(surface);
        self
    }

    pub async fn build(self) -> Result<Adapter, Error> {
        let adapter = self
            .instance
            .request_adapter(&self.request_adapter_options)
            .await
            .ok_or(Error::CreateAdapterError)?;
        Ok(Adapter(adapter))
    }
}
