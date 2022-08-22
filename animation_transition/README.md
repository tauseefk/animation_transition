# animation-transition

Companion macro [here](https://github.com/tauseefk/animation-transition-derive-macro)

Fancy way to describe animation loops in a single array coz cache friendly.
This is a lie, all it does is loop over array indices.
You can probably use it for other things.

![Screen Shot 2022-08-08 at 3 28 21 PM](https://user-images.githubusercontent.com/11029896/183836973-f002f30f-8ac2-4717-8240-b1a9ecb70813.png)

![Screen Shot 2022-08-08 at 3 28 43 PM](https://user-images.githubusercontent.com/11029896/183836987-f1f6dce6-871e-4e5a-8da3-841734043c46.png)

### Usage

1. Create an enum to define animation loops

```Rust
#[derive(Clone, PartialEq)]
pub enum PlayerAnimationVariant {
    Idle,
    Rising,
    Falling,
}
```

2. Implement AnimationLoop for the enum

```Rust
impl AnimationLoop for PlayerAnimationVariant {
    fn page(&self) -> (usize, usize) {
        match self {
            // return values (idx_offset, loop_size) describe the animation loop
            PlayerAnimationVariant::Idle => (0, 3),
            PlayerAnimationVariant::Rising => (2, 2),
            PlayerAnimationVariant::Falling => (4, 4),
        }
    }
}
```

3. Create a struct to store current animation state

```Rust
pub struct PlayerAnimationState {
    pub variant: PlayerAnimationVariant,
    pub idx: usize,
}
```

4. **And finally:**

Option 1. use the handy macro [here](https://github.com/tauseefk/animation-transition-derive-macro)

```Rust
#[derive(AnimationTransitionMacro)]
pub struct PlayerAnimationState {
    /// This is needed to tell the compiler the type of your variant enum
    #[variant]
    pub variant: PlayerAnimationVariant,
    pub idx: usize,
}
```



Option 2. Or Implement `AnimationTransition` for the newly created struct yourself

```Rust
impl AnimationTransition<PlayerAnimationVariant> for PlayerAnimationState {
    fn wrapping_next_idx(&mut self) -> usize {
        // This is example implementation, you can choose to update the page however you like
        let current_idx = self.idx;
        let (offset, size) = self.variant.page();

        self.idx = offset + (current_idx + 1) % size;

        self.idx
    }

    fn transition_variant(&mut self, to: PlayerAnimationVariant) {
        let (offset, _) = to.page();
        self.variant = to;
        self.idx = offset;
    }
}

```
