use dxe_core::ComponentManager;
use r_efi::efi::{protocols::*, Guid};
use sdk::component::params::{Config, ConfigMut, Protocol, Storage};

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

// Access conflict to the same configuration. This will panic when registered.
#[allow(unused)]
fn component0(data: Config<i32>, data2: ConfigMut<i32>) {
    log::info!("This component should have caused a panic when registered.")
}

// Access conflict to the same configuration. This will panic when registered.
#[allow(unused)]
fn component1(data: ConfigMut<i32>, data2: ConfigMut<i32>) {
    log::info!("This component should have caused a panic when registered.")
}

// Access conflict to the same configuration. This will panic when registered.
#[allow(unused)]
fn component2(data: ConfigMut<i32>, data2: Config<i32>) {
    panic!("This component should never run.")
}

// This component should never run, as we never publish the protocol. It can
// be registered without issue.
fn component3(_upd4: Protocol<udp4::Protocol>) {
    panic!("This component should never run.");
}

// This component should run, as we eventually publish the protocol.
fn component4(_rng: Protocol<rng::Protocol>) {
    log::info!("Component 4: Access to the RNG Protocol.");
    log::info!("  This finally ran because something else registered the protocol");
}

// No access conflicts, even if having the same config twice is dumb
fn component5(data: Config<i32>, data2: Config<i32>) {
    log::info!("Component 5: Two immutable access to the same configuration.");
    log::info!("  data: {}", *data);
    log::info!("  data2: {}", *data2);
}

// Access to a configuration that was not registered will provide a default value.
fn component6(data: Config<usize>) {
    log::info!("Component 6: Access to a configuration value that was not registered will provide a default value.");
    log::info!("  data: {}", *data);
}

// Mutable access to a configuration allows us to update the config value. Probably
// not a good idea, and will likely be removed. This is just to show we can have mutable
// access to values.
fn component7(mut data: ConfigMut<i32>) {
    log::info!("Component 7: Mutable access to a configuration value.");
    log::info!("  data: {}", *data);
    (*data) += 1;
    log::info!("  data after change: {}", *data);
}

fn component8(data: Config<i32>) {
    log::info!("Component 8: Showing that the value changed in Component 8 Stuck.");
    log::info!("  data: {}", *data);
}

// We can access the entire storage object. This is not recommended as it requires
// exclusive access to the storage object, meaning no other components can run.
fn component9(_storage: &Storage) {
    log::info!("Component 9: We can access the entire underlying storage object.");
    log::info!("  This grants the component exclusive access. e.g. no other components can run.");
}

// We can have mutable access the entire storage object. This is not recommended as it requires
// exclusive access to the storage object, meaning no other components can run.
//
// Registering a protocol, so we can trigger component 4 on the next run.
fn component10(storage: &mut Storage) {
    log::info!("Component 10: We can mutably access the entire underlying storage object.");
    log::info!("  This grants the component exclusive access. e.g. no other components can run.");

    extern "efiapi" fn get_info(
        _: *mut rng::Protocol,
        _: *mut usize,
        _: *mut Guid,
    ) -> r_efi::efi::Status {
        todo!()
    }

    extern "efiapi" fn get_rng(
        _: *mut rng::Protocol,
        _: *mut Guid,
        _: usize,
        _: *mut u8,
    ) -> r_efi::efi::Status {
        todo!()
    }

    let rng_prot = rng::Protocol {
        get_info: get_info,
        get_rng: get_rng,
    };
    storage.add_protocol(rng_prot);
}

// fn component10()
fn main() {
    std::env::set_var("RUST_LOG", "TRACE");
    colog::init();

    let mut scheduler = ComponentManager::new();

    // All Configuration must implement Default. Attempting to Add a configuration that does not implement Default
    // Will fail. This is mainly to ensure the reverse, however, that creating a component that requires a Config
    // that does not implement Default will fail.
    scheduler.add_config(10i32);

    // scheduler.add_component(component0);
    // scheduler.add_component(component1);
    // scheduler.add_component(component2);
    scheduler.add_component(component3);
    scheduler.add_component(component4);
    scheduler.add_component(component5);
    scheduler.add_component(component6);
    scheduler.add_component(component7);
    scheduler.add_component(component8);
    scheduler.add_component(component9);
    scheduler.add_component(component10);

    log::info!("Components Registered: {}", scheduler.component_count());
    log::info!("");

    log::info!("Running Components:");
    scheduler.run();

    log::info!("");
    log::info!("Components Not Run: {}", scheduler.component_count());
}
