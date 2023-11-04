# IC_Utils
Designed for rapid iteration in IC Canister development projects.


### IcContext
IcContext provides the context for ic canister object invocation, which is used to store the object type and facilitate the user to store and call the object data in canister.

```rust
let tll = context::get_mut::<VecDeque<SwapOrder>>();
```

In this example, an empty VecDeque of type VecDeque<SwapOrder> is returned if no object of type VecDeque<SwapOrder> exists in the context, or a stored object if one exists. The returned object is actually a mutable reference, and any user edits to the reference will modify the object.


### NatUtils
`nat_utils` is used for rapidly generating Nat types as natural numbers and performing mutual conversions between different types.

```rust
 fstr_to_nat("3.1231313",1_000_000_000_000_000_000f64);
```
In this example, we are converting numeric characters of type 'str' into Nat numbers with 18-digit precision.


### guard
The tool supports locking operations requested to canister to prevent anomalies caused by data reentry.
```rust
//Add Exclusion Lock
let guard = CallerGuard::new(id().clone());
    if guard.is_err() {
    panic!("{}",guard.err().unwrap());
}
```
