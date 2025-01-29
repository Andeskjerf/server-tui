use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use sysinfo::System;

use crate::{
    models::{
        event_bus_field_type::EventFieldType, event_bus_message::EventBusMessage,
        event_type::EventType,
    },
    traits::runnable::Runnable,
};

use super::event_bus::EventBus;

pub const EVENT_TOPIC: &str = "hw_usage";

pub struct HwUsageService {
    event_bus: Arc<Mutex<EventBus>>,
    system: Arc<Mutex<System>>,
}

impl HwUsageService {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        Self {
            event_bus,
            system: Arc::new(Mutex::new(System::new_all())),
        }
    }

    fn poll_system(event_bus: Arc<Mutex<EventBus>>, system: Arc<Mutex<System>>) {
        let mut system = system.lock().unwrap();
        (*system).refresh_cpu_usage();
        (*system).refresh_memory();

        let cpu_usage = system.global_cpu_usage() as f64;
        let ram_usage = (system.used_memory() as f64 / system.total_memory() as f64) * 100.0;

        let cpu_bytes = cpu_usage.to_bits().to_le_bytes().to_vec();
        let ram_bytes = ram_usage.to_bits().to_le_bytes().to_vec();

        event_bus.lock().unwrap().publish(
            EVENT_TOPIC,
            EventBusMessage::new(
                "usage",
                EventType::HWusage,
                Some(vec![
                    (EventFieldType::Cpu, cpu_bytes),
                    (EventFieldType::Memory, ram_bytes),
                ]),
            )
            .format_bytes(),
        );
    }
}

impl Runnable for HwUsageService {
    fn run(&self) {
        let event_bus = Arc::clone(&self.event_bus);
        let system = Arc::clone(&self.system);

        tokio::spawn(async move {
            loop {
                HwUsageService::poll_system(Arc::clone(&event_bus), Arc::clone(&system));
                sleep(Duration::from_millis(100));
            }
        });
    }
}
