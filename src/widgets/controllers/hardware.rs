use std::sync::{Arc, Mutex, MutexGuard};

use log::trace;

use crate::{
    models::{event_bus_field_type::EventFieldType, event_bus_message::EventBusMessage},
    services::{event_bus::EventBus, hw_usage},
    utils::bytes_helper::bytes_to_f64,
};

pub struct HardwareUsageController {
    pub ram: Arc<Mutex<Vec<f64>>>,
    pub cpu: Arc<Mutex<Vec<f64>>>,
    pub history: f64,
}

impl HardwareUsageController {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        let ram = Arc::new(Mutex::new(vec![]));
        let cpu = Arc::new(Mutex::new(vec![]));
        let history = 100.0;

        HardwareUsageController::subscribe(event_bus, history, Arc::clone(&cpu), Arc::clone(&ram));

        Self { ram, cpu, history }
    }

    fn subscribe(
        event_bus: Arc<Mutex<EventBus>>,
        limit: f64,
        cpu: Arc<Mutex<Vec<f64>>>,
        ram: Arc<Mutex<Vec<f64>>>,
    ) {
        let mut lock = event_bus.lock().unwrap();

        // watch hw usage
        lock.subscribe(hw_usage::EVENT_TOPIC, move |data| {
            HardwareUsageController::on_event(data, limit, Arc::clone(&cpu), Arc::clone(&ram));
        });
    }

    fn on_event(data: Vec<u8>, limit: f64, cpu: Arc<Mutex<Vec<f64>>>, ram: Arc<Mutex<Vec<f64>>>) {
        let msg = EventBusMessage::from_bytes(data);
        let cpu_usage = bytes_to_f64(msg.get_field(EventFieldType::Cpu));
        let ram_usage = bytes_to_f64(msg.get_field(EventFieldType::Memory));

        let mut cpu_lock = cpu.lock().unwrap();
        let mut ram_lock = ram.lock().unwrap();
        cpu_lock.push(cpu_usage);
        ram_lock.push(ram_usage);

        trace!("adding cpu: {}", cpu_lock.len());
        trace!("adding ram: {}", ram_lock.len());

        if ram_lock.len() > limit as usize {
            cpu_lock.remove(0);
            ram_lock.remove(0);
            trace!("removing");
        }
    }

    pub fn cpu_lock(&self) -> MutexGuard<'_, Vec<f64>> {
        let lock = self.cpu.lock().unwrap();
        trace!("cpu lock len: {}", lock.len());
        lock
    }

    pub fn ram_lock(&self) -> MutexGuard<'_, Vec<f64>> {
        let lock = self.ram.lock().unwrap();
        trace!("ram lock len: {}", lock.len());
        lock
    }
}
