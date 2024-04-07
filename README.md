# animation-transition

### Turn this:
![toad_anime_jump_small](https://user-images.githubusercontent.com/11029896/196866076-e491eb2c-df84-454c-bc3e-302c76feb705.png)

### into this:
![toad_anime_nobg_small](https://user-images.githubusercontent.com/11029896/196866248-5c28ed96-4f96-433b-a1b6-3319c2991a95.gif)

Describe animation loops in a single array.
You can probably use it for other things.

```
                                                                       
                    DECLARING THE ENUM                                 
                                                                       
                                                                       
       ╔═══════╦═══════╦═══════╦═══════╦═══════╦═══════╗               
       ║       ║       ║       ║       ║       ║       ║▐▌             
       ║   0   ║   1   ║   2   ║   3   ║   4   ║   5   ║▐▌             
       ╚═══════╩═══════╩═══════╩═══════╩═══════╩═══════╝▐▌             
        ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘             
                           [usize; 6]                                  
                                                                       
                                                                       
     pub enum Variant {                pub trait AnimationLoop {       
         Idle,           implements                                    
         Rising, ━━━━━━━━━━━━━━━━━━━━━▶    fn page() -> (usize, usize);
         Falling,                                                      
     }                                 }                               
                                                                       
                                                                       
                    MAPPING TO idx/offset                              
                                                                       
       ╔═══════╦═══════╦═══════╦═══════╦═══════╦═══════╗               
       ║       ║       ║       ║       ║       ║       ║▐▌             
       ║   0   ║   1   ║   2   ║   3   ║   4   ║   5   ║▐▌             
       ╚═══════╩═══════╩═══════╩═══╤═══╩═══════╩═══════╝▐▌             
        ▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘             
                   └───────┬───────┘                                   
                           ╵                                           
              Idle::pages(); // (offset: 1, size: 3)                   
                                                                       
                                                                       
                                                                       
       ╔═══════╦═══════╦═══════╦═══════╦═══════╦═══════╗               
       ║       ║       ║       ║       ║       ║       ║▐▌             
       ║   0   ║   1   ║   2   ║   3   ║   4   ║   5   ║▐▌             
       ╚═══════╩═══════╩═══════╩═══╤═══╩═══╤═══╩═══════╝▐▌             
        ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▘             
                                   └───┬───┘                           
                                       ╵                               
              Rising::pages(); // (offset: 3, size: 2)                 
                                                                       
                                                                       
```

```
                                                                       
                                                                       
                   FLIPPING THROUGH SPRITES                            
                                                                       
                                                                       
                                   ┌───────┐                           
                                   │       │                           
       ╔═══════╦═══════╦═══════╦═══╧═══╦═══▼═══╦═══════╗               
       ║       ║       ║       ║       ║       ║       ║▐▌             
       ║   0   ║   1   ║   2   ║   3   ║   4   ║   5   ║▐▌             
       ╚═══════╩═══════╩═══════╩═══════╩═══════╩═══════╝▐▌             
        ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘             
                                                                       
              animation_state.variant; // Variant::Rising              
              animation_state.idx; // 3                                
              animation_state.wrapping_next_idx(); //  4               
                                                                       
                                                                       
                       VARIANT TRANSITION                              
                                                                       
                                                                       
                                       ┏━━━━━━━━━━━┓                   
                                       ┃           ┃                   
                                   ┌───┸───┐       ┃                   
       ╔═══════╦═══════╦═══════╦═══╧═══╦═══╧═══╦═══▼═══╗               
       ║       ║       ║       ║       ║       ║       ║▐▌             
       ║   0   ║   1   ║   2   ║   3   ║   4   ║   5   ║▐▌             
       ╚═══════╩═══════╩═══════╩═══════╩═══════╩═══════╝▐▌             
        ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘             
                                                                       
              animation_state.variant;              // Variant::Rising 
              animation_state.transition_variant(); // ()              
              animation_state.variant;              // Variant::Falling
              animation_state.idx;                  // 5               
                                                                       
                                                                       
```

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

Option 1. use the handy macro:
enable by using the `derive` feature, in `Cargo.toml`

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
