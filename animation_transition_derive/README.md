# animation-transition-derive

This is a companion macro for [animation-transition](https://github.com/tauseefk/animation-transition).

![Screen Shot 2022-08-08 at 3 28 21 PM](https://user-images.githubusercontent.com/11029896/183836973-f002f30f-8ac2-4717-8240-b1a9ecb70813.png)

![Screen Shot 2022-08-08 at 3 28 43 PM](https://user-images.githubusercontent.com/11029896/183836987-f1f6dce6-871e-4e5a-8da3-841734043c46.png)

### Usage

```Rust
#[derive(AnimationTransitionMacro)]
pub struct PlayerAnimationState {
    /// This is needed to tell the compiler the type of your animation variant enum
    #[variant]
    pub variant: PlayerAnimationVariant,
    pub idx: usize,
}
```
