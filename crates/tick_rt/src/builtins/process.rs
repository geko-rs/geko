/// Imports
use crate::{
    builtins::utils,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc, thread, time::Duration};

/// Thread sleep
fn sleep() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(time) => {
                if *time >= 0 {
                    thread::sleep(Duration::from_millis(*time as u64));
                    Value::Null
                } else {
                    utils::error(span, "time should be >= 0")
                }
            }
            _ => utils::error(span, "time is not an int"),
        }),
    });
}

/// Provides `is` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();
    env.force_define("sleep", Value::Callable(Callable::Native(sleep())));
    Rc::new(RefCell::new(env))
}
