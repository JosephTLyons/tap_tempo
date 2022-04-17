# tap_tempo

Calculate tempos by simply tapping

```rust
let mut tap_tempo = TapTempo::new();
let tempo = tap_tempo.tap();
assert!(tempo.is_none());

// After some time has passed ...

let tempo = tap_tempo.tap();
assert!(tempo.is_some());
```
