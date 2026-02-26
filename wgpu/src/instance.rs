use wgpu::{Backends, Instance, InstanceFlags};

#[cfg(not(target_arch = "wasm32"))]
use dawn_rs::{InstanceDescriptor, InstanceFeatureName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    Wgpu,
    Dawn,
}

pub async fn create_instance(
    runtime: Runtime,
    backends: Backends,
    flags: InstanceFlags,
) -> Option<Instance> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = runtime;

        Some(
            wgpu::util::new_instance_with_webgpu_detection(&wgpu::InstanceDescriptor {
                backends,
                flags,
                ..Default::default()
            })
            .await,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        match runtime {
            Runtime::Wgpu => Some(wgpu::util::new_instance_with_webgpu_detection(
                &wgpu::InstanceDescriptor {
                    backends,
                    flags,
                    ..Default::default()
                },
            )
            .await),
            Runtime::Dawn => {
                let _ = (backends, flags);

                let mut descriptor = InstanceDescriptor::new();
                descriptor.required_features = Some(vec![InstanceFeatureName::TimedWaitAny]);

                let instance = dawn_rs::Instance::new(Some(&descriptor));
                Some(dawn_wgpu::Compat::from(instance).into())
            }
        }
    }
}
