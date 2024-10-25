pub trait Subscriber: Send + Sync {
    fn notify_rising_edge(&self);
    fn notify_falling_edge(&self);
    fn run(&mut self);
}

pub struct Clock {
    half_cycle_time_s: f64,
    subscribers: Vec<*const dyn Subscriber>,
}

unsafe impl Send for Clock {} //TODO: can it be done better?
unsafe impl Sync for Clock {} //TODO: can it be done better?

impl Clock {
    pub fn new(frequency_hz: f64) -> Self {
        Self {
            half_cycle_time_s: 1f64 / frequency_hz / 2f64,
            subscribers: vec![],
        }
    }

    pub fn run(&self) {
        std::thread::sleep(std::time::Duration::from_secs_f64(self.half_cycle_time_s));

        self.notify_rising_edge();

        std::thread::sleep(std::time::Duration::from_secs_f64(self.half_cycle_time_s));

        self.notify_falling_edge();
    }

    fn notify_rising_edge(&self) {
        for subscriber in &self.subscribers {
            unsafe { subscriber.as_ref().unwrap().notify_rising_edge() }; //TODO: Can it be done better?
        }
    }

    fn notify_falling_edge(&self) {
        for subscriber in &self.subscribers {
            unsafe { subscriber.as_ref().unwrap().notify_falling_edge() }; //TODO: Can it be done better?
        }
    }

    pub fn subscribe(&mut self, subscriber: *const dyn Subscriber) {
        self.subscribers.push(subscriber);
    }
}

#[cfg(test)]
mod tests {
    use super::{Clock, Subscriber};

    struct MockSubscriber {
        pub rising_edge_timestamp_ms: std::sync::atomic::AtomicI64,
        pub falling_edge_timestamp_ms: std::sync::atomic::AtomicI64,
        pub expected_frequency_hz: f64,
    }
    impl Subscriber for MockSubscriber {
        fn notify_rising_edge(&self) {
            self.rising_edge_timestamp_ms.store(
                std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        fn notify_falling_edge(&self) {
            self.falling_edge_timestamp_ms.store(
                std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        fn run(&mut self) {
            let start_time = self
                .rising_edge_timestamp_ms
                .load(std::sync::atomic::Ordering::Relaxed);

            let end_time = self
                .falling_edge_timestamp_ms
                .load(std::sync::atomic::Ordering::Relaxed);

            let measured_half_cycle_time_ms = i64::abs(start_time - end_time);

            let expected_half_cycle_time_ms =
                (1.0 / self.expected_frequency_hz / 2.0 * 1000.0) as i64;

            // the error should be less than 5ms
            assert!(i64::abs(measured_half_cycle_time_ms - expected_half_cycle_time_ms) < 5);
        }
    }

    #[test]
    fn test_run() {
        let requested_frequency_hz = 1.0;

        let mut mock_subscriber = MockSubscriber {
            rising_edge_timestamp_ms: std::sync::atomic::AtomicI64::new(0),
            falling_edge_timestamp_ms: std::sync::atomic::AtomicI64::new(0),
            expected_frequency_hz: requested_frequency_hz,
        };

        let mut clock = Clock::new(requested_frequency_hz);
        clock.subscribe(&mock_subscriber);

        clock.run(); // run single clock cycle
        mock_subscriber.run(); // check if it was as expected
    }
}
