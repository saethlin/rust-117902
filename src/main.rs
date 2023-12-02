use wgpu::util::DeviceExt;

fn main() {
    let async_renderer = async move {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&Default::default())
            .await
            .expect("couldn't get instance");

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .expect("couldn't get device and queue");

        let texture_descriptor = wgpu::TextureDescriptor {
            size: Default::default(),
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            dimension: wgpu::TextureDimension::D2,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        device.create_texture_with_data(&queue, &texture_descriptor, &[255; 10]);
        panic!("non-deterministic failure should have happened above");
    };

    // Apparently, the executor needs to be multithreaded for this to reproduce.
    futures_lite::future::block_on(async_renderer);
}
