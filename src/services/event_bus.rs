use std::collections::HashMap;

type Callback = dyn Fn(Vec<u8>) + Send + 'static;

pub struct EventBus {
    subscribers: HashMap<String, Vec<(i32, Box<Callback>)>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    fn get_free_id(&self, event_type: &str) -> i32 {
        let subscribers = match self.subscribers.get(event_type) {
            Some(res) => res,
            None => return 0,
        };

        // could this me more efficient? do we need to iterate through all IDs?
        // use tokens instead?
        let mut free_id: i32 = 0;
        for (id, _) in subscribers {
            if *id == free_id {
                free_id += 1;
            }
        }

        free_id
    }

    pub fn publish(&self, event_type: &str, payload: Vec<u8>) {
        if let Some(subs) = self.subscribers.get(event_type) {
            for (_, callback) in subs {
                callback(payload.clone());
            }
        }
    }

    /// Returns the assigned ID for the callback
    pub fn subscribe<F: Fn(Vec<u8>) + Send + 'static>(
        &mut self,
        event_type: &str,
        callback: F,
    ) -> i32 {
        if !self.subscribers.contains_key(event_type) {
            self.subscribers.insert(event_type.to_string(), vec![]);
        }

        let free_id = self.get_free_id(event_type);
        self.subscribers
            .get_mut(event_type)
            .expect("no vec in HashMap, for some reason!!")
            .push((free_id, Box::new(callback)));
        free_id
    }

    /// Returns the callbacks ID if removed, None otherwise
    pub fn unsubscribe(&mut self, event_type: &str, id: i32) -> Option<i32> {
        let subscribers = match self.subscribers.get_mut(event_type) {
            Some(res) => res,
            None => return None,
        };

        let found_pos = match subscribers
            .iter()
            .position(|(callback_id, _)| *callback_id == id)
        {
            Some(pos) => pos,
            None => return None,
        };

        Some(subscribers.remove(found_pos).0)
    }
}
