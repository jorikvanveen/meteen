use sea_orm::{ActiveValue, Value};

pub mod errors;

pub fn option_to_active<T: Into<Value>>(opt: Option<T>) -> ActiveValue<T> {
    match opt {
        Some(value) => ActiveValue::Set(value),
        None => ActiveValue::NotSet,
    }
}

//pub fn option_to_active_unch<T: Into<Value>>(opt: Option<T>) -> ActiveValue<T> {
//    match opt {
//        Some(value) => ActiveValue::Set(value),
//        None => ActiveValue::Unchanged(())
//    }
//}
