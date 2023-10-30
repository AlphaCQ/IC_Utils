use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::ops::{Div, Mul};
use bigdecimal::{BigDecimal, ToPrimitive};
use candid::{Nat, Principal};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};

use ic_cdk;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use num_bigint::BigUint;

static mut CONTEXT: Option<IcContext> = None;

/// A singleton context that is used in the actual IC environment.
pub struct IcContext {
    /// The storage for this context.
    storage: BTreeMap<TypeId, Box<dyn Any>>,
}

impl IcContext {
    /// Return a mutable reference to the context.
    #[inline(always)]
    pub fn context() -> &'static mut IcContext {
        unsafe {
            if let Some(ctx) = &mut CONTEXT {
                ctx
            } else {
                CONTEXT = Some(IcContext {
                    storage: BTreeMap::new(),
                });
                IcContext::context()
            }
        }
    }

    #[inline(always)]
    pub fn as_mut(&self) -> &mut Self {
        unsafe {
            let const_ptr = self as *const Self;
            let mut_ptr = const_ptr as *mut Self;
            &mut *mut_ptr
        }
    }
    #[inline(always)]
    pub fn trap(&self, message: &str) -> ! {
        ic_cdk::api::trap(message);
    }

    #[inline(always)]
    pub fn print<S: std::convert::AsRef<str>>(&self, s: S) {
        ic_cdk::api::print(s)
    }

    #[inline(always)]
    pub fn id(&self) -> Principal {
        ic_cdk::id()
    }

    #[inline(always)]
    pub fn time(&self) -> u64 {
        ic_cdk::api::time()
    }

    #[inline(always)]
    pub fn balance(&self) -> u64 {
        ic_cdk::api::canister_balance()
    }

    #[inline(always)]
    pub fn caller(&self) -> Principal {
        ic_cdk::api::caller()
    }

    #[inline(always)]
    pub fn msg_cycles_available(&self) -> u64 {
        ic_cdk::api::call::msg_cycles_available()
    }

    #[inline(always)]
    pub fn msg_cycles_accept(&self, amount: u64) -> u64 {
        ic_cdk::api::call::msg_cycles_accept(amount)
    }

    #[inline(always)]
    pub fn msg_cycles_refunded(&self) -> u64 {
        ic_cdk::api::call::msg_cycles_refunded()
    }

    #[inline(always)]
    pub fn store<T: 'static>(&self, data: T) {
        let type_id = TypeId::of::<T>();
        self.as_mut().storage.insert(type_id, Box::new(data));
    }

    #[inline]
    pub fn get_maybe<T: 'static>(&self) -> Option<&T> {
        let type_id = std::any::TypeId::of::<T>();
        self.storage
            .get(&type_id)
            .map(|b| b.downcast_ref().expect("Unexpected value of invalid type."))
    }

    #[inline(always)]
    pub fn get_mut<T: 'static + Default>(&self) -> &mut T {
        let type_id = std::any::TypeId::of::<T>();
        self.as_mut()
            .storage
            .entry(type_id)
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut()
            .expect("Unexpected value of invalid type.")
    }

    #[inline(always)]
    pub fn delete<T: 'static + Default>(&self) -> bool {
        let type_id = std::any::TypeId::of::<T>();
        self.as_mut().storage.remove(&type_id).is_some()
    }

    #[inline(always)]
    pub fn stable_store<T>(&self, data: T) -> Result<(), candid::Error>
        where
            T: ArgumentEncoder,
    {
        ic_cdk::storage::stable_save(data)
    }

    #[inline(always)]
    pub fn stable_restore<T>(&self) -> Result<T, String>
        where
            T: for<'de> ArgumentDecoder<'de>,
    {
        ic_cdk::storage::stable_restore()
    }


    #[inline(always)]
    pub fn set_certified_data(&self, data: &[u8]) {
        ic_cdk::api::set_certified_data(data);
    }

    #[inline(always)]
    pub fn data_certificate(&self) -> Option<Vec<u8>> {
        ic_cdk::api::data_certificate()
    }

    #[inline(always)]
    pub fn spawn<F: 'static + std::future::Future<Output=()>>(&mut self, future: F) {
        ic_cdk::spawn(future)
    }
}


#[inline(always)]
fn get_context() -> &'static mut IcContext {
    return IcContext::context();
}

/// Trap the code.
#[inline(always)]
pub fn trap(message: &str) -> ! {
    get_context().trap(message)
}

/// Print a message.
#[inline(always)]
pub fn print<S: std::convert::AsRef<str>>(s: S) {
    get_context().print(s)
}

/// ID of the current canister.
#[inline(always)]
pub fn id() -> Principal {
    get_context().id()
}

/// The time in nanoseconds.
#[inline(always)]
pub fn time() -> u64 {
    get_context().time()
}

/// The balance of the canister.
#[inline(always)]
pub fn balance() -> u64 {
    get_context().balance()
}

/// The caller who has invoked this method on the canister.
#[inline(always)]
pub fn caller() -> Principal {
    get_context().caller()
}

/// Return the number of available cycles that is sent by the caller.
#[inline(always)]
pub fn msg_cycles_available() -> u64 {
    get_context().msg_cycles_available()
}

/// Accept the given amount of cycles, returns the actual amount of accepted cycles.
#[inline(always)]
pub fn msg_cycles_accept(amount: u64) -> u64 {
    get_context().msg_cycles_accept(amount)
}

/// Return the cycles that were sent back by the canister that was just called.
/// This method should only be called right after an inter-canister call.
#[inline(always)]
pub fn msg_cycles_refunded() -> u64 {
    get_context().msg_cycles_refunded()
}

/// Store the given data to the storage.
#[inline(always)]
pub fn store<T: 'static>(data: T) {
    get_context().store(data)
}

/// Return the data that does not implement [`Default`].
#[inline(always)]
pub fn get_maybe<T: 'static>() -> Option<&'static T> {
    get_context().get_maybe()
}

/// Return the data associated with the given type. If the data is not present the default
/// value of the type is returned.
#[inline(always)]
pub fn get<T: 'static + Default>() -> &'static T {
    get_context().get_mut()
}

/// Return a mutable reference to the given data type, if the data is not present the default
/// value of the type is constructed and stored. The changes made to the data during updates
/// is preserved.
#[inline(always)]
pub fn get_mut<T: 'static + Default>() -> &'static mut T {
    get_context().get_mut()
}

/// Remove the data associated with the given data type.
#[inline(always)]
pub fn delete<T: 'static + Default>() -> bool {
    get_context().delete::<T>()
}

/// Store the given data to the stable storage.
#[inline(always)]
pub fn stable_store<T>(data: T) -> Result<(), candid::Error>
    where
        T: ArgumentEncoder,
{
    get_context().stable_store(data)
}

/// Restore the data from the stable storage. If the data is not already stored the None value
/// is returned.
#[inline(always)]
pub fn stable_restore<T>() -> Result<T, String>
    where
        T: for<'de> ArgumentDecoder<'de>,
{
    get_context().stable_restore()
}

/// Set the certified data of the canister, this method traps if data.len > 32.
#[inline(always)]
pub fn set_certified_data(data: &[u8]) {
    get_context().set_certified_data(data)
}

/// Returns the data certificate authenticating certified_data set by this canister.
#[inline(always)]
pub fn data_certificate() -> Option<Vec<u8>> {
    get_context().data_certificate()
}

/// Execute a future without blocking the current call.
#[inline(always)]
pub fn spawn<F: 'static + std::future::Future<Output=()>>(future: F) {
    get_context().spawn(future)
}


pub fn nat_15() -> Nat {
    Nat::from(1_000_000_000_000_000u64)
}

pub fn nat_8() -> Nat {
    Nat::from(100_000_000u64)
}

pub fn nat_18() -> Nat {
    Nat::from(1_000_000_000_000_000_000u64)
}

pub fn nat_12() -> Nat {
    Nat::from(1_000_000_000_000u64)
}

pub fn nat_36() -> Nat {
    Nat::from(1_000_000_000_000_000_000_000_000_000_000_000_000u128)
}

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

pub fn fstr_to_nat(v: String, decimal: f64) -> Nat {
    let l: f64 = v.parse::<f64>().unwrap() * decimal;
    Nat::from(l as u64)
}

pub fn nat_from(v: u64) -> Nat {
    Nat::from(v)
}

pub fn new_zero() -> Nat {
    nat_from(0)
}

pub fn div_to_f64(value: Nat, decimals: u8) -> f64 {
    let value = BigDecimal::from(nat_to_u128(value)).div(10_u64.pow(decimals as u32)).to_f64().unwrap_or(0f64);
    value
}

pub fn calc_value_to_f64(value: Nat, price: Nat, decimals: u8) -> f64 {
    return (BigDecimal::from(nat_to_u128(value)).div(10_u64.pow(decimals as u32))).mul(
        BigDecimal::from(nat_to_u128(price)).div(10_u64.pow(18u32))
    ).to_f64().unwrap_or(0f64);
}

pub fn nat_to_u64(value: Nat) -> u64 {
    let v: u64 = BigUint::from(value).try_into().unwrap();
    v
}

pub fn nat_to_u128(value: Nat) -> u128 {
    let v: u128 = BigUint::from(value).try_into().unwrap();
    v
}

pub fn nat_to_u8(value: Nat) -> u8 {
    let v: u8 = BigUint::from(value).try_into().unwrap();
    v
}

pub fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg, )).expect("Failed to encode arguments.");
    // The coefficients can be found in [this page](https://internetcomputer.org/docs/current/developer-docs/production/computation-and-storage-costs).
    // 12 is "http_request".len().
    400_000_000u128 + 100_000u128 * (arg_raw.len() as u128 + 12 + max_response_bytes)
}
