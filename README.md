# IC_Utils
Designed for rapid iteration in IC Canister development projects.


### IcContext
IcContext provides the context for ic canister object invocation, which is used to store the object type and facilitate the user to store and call the object data in canister.

```rust
let tll = ic_utils::get_mut::<VecDeque<SwapOrder>>();
```

In this example, an empty VecDeque of type VecDeque<SwapOrder> is returned if no object of type VecDeque<SwapOrder> exists in the context, or a stored object if one exists. The returned object is actually a mutable reference, and any user edits to the reference will modify the object.