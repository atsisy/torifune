pub type Clock = u64;

pub struct ScheduledEvent<Args> {
    run_time: Clock,
    func: fn(Args) -> Result<(), &'static str>,
}

impl<Args> ScheduledEvent<Args> {

    /// ScheduledEvent構造体の生成メソッド 
    ///
    /// # Example
    /// ```
    ///     use trojan::core::*;
    ///
    ///     let event = ScheduledEvent::new(|x: i32| {
    ///         println!("Event is Called {}", x);
    ///         Ok(())
    ///     }, 10);
    /// ```
    pub fn new(func: fn(Args) ->  Result<(), &'static str>, call_abs: Clock) -> ScheduledEvent<Args> {
        ScheduledEvent { run_time: call_abs, func: func }
    }
    
    pub fn call_event(&self, args: Args) -> Result<(), &'static str> {
        (self.func)(args)
    }

    pub fn get_scheduled(&self) -> Clock {
        self.run_time
    }
    
}

pub trait Updatable {
    fn update(&mut self, ctx: &ggez::Context, t: Clock) -> Result<(), &'static str> {
        Ok(())
    }
}
