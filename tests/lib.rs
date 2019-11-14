extern crate torifune;
use torifune::core::*;

#[test]
fn check_scheduled_event() {
    let event = ScheduledEvent::new(&|x: i32| {
        println!("Event is Called {}", x);
        Ok(())
    }, 10);

    match event.call_event(10) {
        Ok(_result) => println!("OK"),
        Err(msg) => println!("{}", msg),
    }
    
    assert_eq!(10, event.get_scheduled());
}
