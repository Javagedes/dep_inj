use dep_inj::{ComponentManager, params::{Config, ConfigMut, Service}};

fn component1() {
    println!("Component 1");
}

fn component2(mut data: ConfigMut<i32>) {
    println!("Component 2: {}", *data);
    *data += 1;
}

fn component3(data: Config<i32>, data2: Config<f32>) {
    println!("Component 3: {} {}", *data, *data2);
}

// An example of registering a trait object
trait MyTrait {
    fn print(&self);
}

impl MyTrait for i32 {
    fn print(&self) {
        println!("Trait for i32: {}", self);
    }
}

fn component4(data: Service<&dyn MyTrait>) {
    (*data).print();
}

trait MyTrait2 {}

fn component5(data: Service<&dyn MyTrait2>) {
    println!("This should not run");
}

fn main() {

    let mut scheduler = ComponentManager::default();

    // All Configuration must implement Default. Attempting to Add a configuration that does not implement Default
    // Will fail. This is mainly to ensure the reverse, however, that creating a component that requires a Config
    // that does not implement Default will fail.
    scheduler.add_config(32i32);
    scheduler.add_config(3.14f32);

    // Services are not required to implement Default. If they do not exist, and are requested by a component,
    // the component will not run.
    scheduler.add_service(&32i32 as &dyn MyTrait);

    scheduler.add_component(component1);
    scheduler.add_component(component2);
    scheduler.add_component(component3);
    scheduler.add_component(component4);
    scheduler.add_component(component5);

    println!("Components Registered: {}\n", scheduler.components.len());

    println!("Running Components:");
    scheduler.run();

    println!("");
    println!("Components Not Run: {}", scheduler.components.len());
}
