use ic_cdk::api::call::CallResult;

pub fn require<T>(cod: bool, e: T) -> Result<(), T> {
    if !cod {
        return Err(e);
    }
    return Ok(());
}

pub fn throw<V>(r: Result<V, String>) -> V {
    match r {
        Ok(v) => { v }
        Err(e) => { panic!("{}", e) }
    }
}

pub fn throw_call_res<V>(r: CallResult<V>, msg: &str) -> V {
    match r {
        Ok(v) => { v }
        Err(e) => { panic!("[{}] error: {}", msg, e.1) }
    }
}