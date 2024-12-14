use dxe_core::ComponentManager;
use sdk::component::params::{Config, Service};
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
    println!("Component 1: A Component with no dependencies");
}

fn component2(data: Config<i32>) {
    println!("Component 2: A Component with a Config dependency");
    println!("  config: {}i32", *data,);
}

fn component3(data: Config<i32>, service: Service<&dyn TestService>) {
    println!("Component 3: A Component with a Config and Service dependency");
    println!("  config: {}i32", *data);
    println!("  Test Service: {}i32", service.increment(*data));
}

fn component4(data: Config<i32>, service: Service<&dyn TestService2>) {
    println!("Component 4: A Component with a Config and Service dependency");
    println!("  config: {}i32", *data);
    println!("  Test Service: {}i32", service.decrement(*data));
}


fn main() {

    let mut scheduler = ComponentManager::default();

    // All Configuration must implement Default. Attempting to Add a configuration that does not implement Default
    // Will fail. This is mainly to ensure the reverse, however, that creating a component that requires a Config
    // that does not implement Default will fail.
    scheduler.add_config(10i32);

    // Services are not required to implement Default. If they do not exist, and are requested by a component,
    // the component will not run.
    scheduler.add_service(&1 as &dyn TestService);

    scheduler.add_component(component1);
    scheduler.add_component(component2);
    scheduler.add_component(component3);
    // This component will not run, as the service is not registered.
    scheduler.add_component(component4);

    println!("Components Registered: {}\n", scheduler.components.len());

    println!("Running Components:");
    scheduler.run();

    println!("");
    println!("Components Not Run: {}", scheduler.components.len());
}
