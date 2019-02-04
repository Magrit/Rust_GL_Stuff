use gfx_hal::{Adapter, Backend, Limits, MemoryType, PhysicalDevice};

pub struct AdapterState<B: Backend> {
  pub adapter: Option<Adapter<B>>,
  memory_types: Vec<MemoryType>,
  limits: Limits,
}

impl<B: Backend> AdapterState<B> {
  pub fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
    print!("Found adapters: ");

    for adapter in adapters.iter() {
      println!("{:?}", adapter.info);
    }

    AdapterState::<B>::new_adapter(adapters.remove(0))
  }

  fn new_adapter(adapter: Adapter<B>) -> Self {
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let limits = adapter.physical_device.limits();
    println!("{:?}", limits);

    AdapterState {
      adapter: Some(adapter),
      memory_types,
      limits,
    }
  }
}