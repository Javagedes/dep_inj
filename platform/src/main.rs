use dxe_core::ComponentManager;
use sdk::component::params::{Config, ConfigMut};
trait TestService {
    fn increment(&self, v: i32) -> i32;
}

trait TestService2 {
    fn decrement(&self, v: i32) -> i32;
}

impl TestService for i32 {
    fn increment(&self, v: i32) -> i32 {
        v + *self
    }
}

fn component1() {
    log::info!("Component 1: A Component with no dependencies");
}

fn component2(data: Config<i32>) {
    log::info!("Component 2: A Component with a Config dependency");
    log::info!("  config: {}i32", *data,);
}

fn component3(data: Config<i32>, data2: Config<f32>) {
    log::info!("Component 3: A Component with multiple Config dependencies");
    log::info!("  config: {}i32", *data);
    log::info!("  config: {}f32", *data2);
}

fn component4(data: ConfigMut<i32>, data2: Config<f32>) {
    log::info!("Component 4: A Component with A mutable Config dependency");
    log::info!("  config(mut): {}i32", *data);
    log::info!("  config: {}f32", *data2);
}

fn component5(mut data: ConfigMut<i32>, mut data2: ConfigMut<f32>) {
    log::info!("Component 5: A Component with multiple mutable Config dependencies");
    log::info!("  config(mut): {}i32", *data);
    log::info!("  config(mut): {}f32", *data2);
    log::info!(" Updating values...");
    (*data) += 1;
    (*data2) += 1.0;
    log::info!("  config(mut): {}i32", *data);
    log::info!("  config(mut): {}f32", *data2);
}

fn component6(data: Config<i32>, data2: Config<f32>) {
    log::info!("Component 6: A Component with a Config dependency");
    log::info!("  config: {}i32", *data);
    log::info!("  config: {}f32", *data2);
}

#[allow(unused)]
fn component7(data: Config<i32>, data2: ConfigMut<i32>) {
    log::info!("This component should have caused a panic when registered.")
}

#[allow(unused)]
fn component8(data: ConfigMut<i32>, data2: ConfigMut<i32>) {
    log::info!("This component should have caused a panic when registered.")
}

#[allow(unused)]
fn component9(data: ConfigMut<i32>, data2: Config<i32>) {
    log::info!("This component should have caused a panic when registered.")
}

fn main() {
    std::env::set_var("RUST_LOG", "TRACE");
    colog::init();

    let mut scheduler = ComponentManager::new();

    // All Configuration must implement Default. Attempting to Add a configuration that does not implement Default
    // Will fail. This is mainly to ensure the reverse, however, that creating a component that requires a Config
    // that does not implement Default will fail.
    scheduler.add_config(10i32);

    scheduler.add_component(component1);
    scheduler.add_component(component2);
    scheduler.add_component(component3);
    scheduler.add_component(component4);
    scheduler.add_component(component5);
    scheduler.add_component(component6);
    // scheduler.add_component(component7);
    // scheduler.add_component(component8);
    // scheduler.add_component(component9);

    log::info!("Components Registered: {}", scheduler.component_count());
    log::info!("");

    log::info!("Running Components:");
    scheduler.run();

    log::info!("");
    log::info!("Components Not Run: {}", scheduler.component_count());
}
