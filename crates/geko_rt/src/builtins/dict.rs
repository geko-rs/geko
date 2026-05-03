/// Imports
use crate::{
    builtin_class,
    builtins::{list::make_list, utils},
    interpreter::Interpreter,
    native_class, native_method,
    refs::{MutRef, Ref},
    rt::value::{Class, Instance, Method, Native, Value},
};
use geko_common::bug;
use geko_lex::token::Span;
use std::{cell::RefCell, collections::HashMap};

/// Helper: validates dict
fn validate_dict<F, V>(span: &Span, dict: Value, f: F) -> V
where
    F: FnOnce(&mut HashMap<Value, Value>) -> V,
{
    match dict {
        Value::Instance(instance) => {
            // Safety: borrow is temporal for this line
            let internal = instance
                .borrow_mut()
                .fields
                .get("$internal")
                .cloned()
                .unwrap();

            match internal {
                Value::Any(map) => match map.borrow_mut().downcast_mut::<HashMap<Value, Value>>() {
                    Some(map) => f(map),
                    _ => utils::error(span, "corrupted dict"),
                },
                _ => {
                    utils::error(span, "corrupted dict");
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates dict argument
fn validate_dict_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(&mut HashMap<Value, Value>) -> V,
{
    validate_dict(span, values.first().cloned().unwrap(), f)
}

/// Helper: makes new dict
#[allow(dead_code)]
pub fn make_dict(rt: &mut Interpreter, span: &Span) -> MutRef<Instance> {
    // Getting builtin class
    let class = builtin_class!(rt, "Dict");

    // Calling class
    match rt.call_class(span, Vec::new(), class) {
        Ok(Value::Instance(instance)) => instance,
        Ok(_) => unreachable!(),
        Err(err) => {
            bug!(format!(
                "calling of builtin `Dict` has ended with a control flow leak: {err:?}"
            ))
        }
    }
}

/// Init method
fn init_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, _, values| {
            let dict = values.first().cloned().unwrap();
            match dict {
                Value::Instance(instance) => {
                    let vec = Value::Any(MutRef::new(RefCell::new(HashMap::<Value, Value>::new())));

                    // Safety: borrow is temporal for this line
                    instance
                        .borrow_mut()
                        .fields
                        .insert("$internal".to_string(), vec);

                    Value::Null
                }
                _ => unreachable!(),
            }
        }
    }
}

/// To string method
fn to_string_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| Value::String(format!("{map:?}")))
        }
    }
}

/// Get method
fn get_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.get(&values.get(1).cloned().unwrap())
                    .cloned()
                    .unwrap_or(Value::Null)
            })
        }
    }
}

/// Insert method
fn insert_method() -> Method {
    native_method! {
        arity = 3,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.insert(
                    values.get(1).cloned().unwrap(),
                    values.get(2).cloned().unwrap(),
                );
                Value::Null
            })
        }
    }
}

/// Remove method
fn remove_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.remove(&values.first().cloned().unwrap());
                Value::Null
            })
        }
    }
}

/// Len method
fn len_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| Value::Int(map.len() as i64))
        }
    }
}

/// Clear method
fn clear_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.clear();
                Value::Null
            })
        }
    }
}

/// Contains key
fn contains_key_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_dict_arg(span, &values, |map| {
                Value::Bool(map.contains_key(values.get(1).unwrap()))
            })
        }
    }
}

/// Keys list
fn keys_method() -> Method {
    native_method! {
        arity = 1,
        fun = |rt, span, values| {
            validate_dict_arg(span, &values, |map| {
                // Preparing keys vector
                let keys = map.keys().cloned().collect::<Vec<Value>>();

                // Preparing list for keys
                let list = make_list(rt, span);

                // Setting new vector
                list.borrow_mut().fields.insert(
                    "$internal".to_string(),
                    Value::Any(MutRef::new(RefCell::new(keys))),
                );

                Value::Instance(list)
            })
        }
    }
}

/// Values list
fn values_method() -> Method {
    native_method! {
        arity = 1,
        fun = |rt, span, values| {
            validate_dict_arg(span, &values, |map| {
                // Preparing values vector
                let values = map.keys().cloned().collect::<Vec<Value>>();

                // Preparing list for keys
                let list = make_list(rt, span);

                // Setting new vector
                list.borrow_mut().fields.insert(
                    "$internal".to_string(),
                    Value::Any(MutRef::new(RefCell::new(values))),
                );

                Value::Instance(list)
            })
        }
    }
}

/// Provides dict class
pub fn provide_class() -> Ref<Class> {
    native_class! {
        name = Dict,
        methods = {
            init => init_method(),
            to_string => to_string_method(),
            get => get_method(),
            insert => insert_method(),
            remove => remove_method(),
            len => len_method(),
            clear => clear_method(),
            contains_key => contains_key_method(),
            keys => keys_method(),
            values => values_method()
        }
    }
}
