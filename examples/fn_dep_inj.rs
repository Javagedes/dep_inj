use dep_inj::{ComponentManager, params::{Config, ConfigMut}};

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

fn component4(data: Config<&dyn MyTrait>) {
    data.print();
}

fn main() {

    let mut scheduler = ComponentManager::default();

    scheduler.add_config(32i32);
    scheduler.add_config(3.14f32);
    scheduler.add_config(&32i32 as &dyn MyTrait);

    scheduler.add_component(component1);
    scheduler.add_component(component2);
    scheduler.add_component(component3);
    scheduler.add_component(component4);

    scheduler.run();
}
