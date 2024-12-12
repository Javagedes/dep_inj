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

fn main() {
    let mut scheduler = ComponentManager::default();
    scheduler.add_config(32i32);
    scheduler.add_config(3.14f32);

    scheduler.add_component(component1);
    scheduler.add_component(component2);
    scheduler.add_component(component3);

    scheduler.run();
}
