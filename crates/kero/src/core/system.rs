use crate::core::{Context, GameError};
use crate::gfx::Draw;
use std::any::Any;

/// A top-level system that runs with the game loop, added via [`new_game()`](crate::new_game).
pub trait System: 'static {
    /// Optional configuration that can be passed into [new()](System::new).
    type Config;

    /// Called when the system starts to load any startup assets and create your system state.
    #[allow(unused_variables)]
    fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized;

    /// Called before every update in order to perform system logic.
    #[allow(unused_variables)]
    fn pre_update(&mut self, ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    /// Called after every update in order to perform system logic.
    #[allow(unused_variables)]
    fn post_update(&mut self, ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    /// Called before every frame refresh in order to perform system rendering.
    #[allow(unused_variables)]
    fn pre_render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        Ok(())
    }

    /// Called after every frame refresh in order to perform system rendering.
    #[allow(unused_variables)]
    fn post_render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        Ok(())
    }
}

pub(crate) struct SystemInfo {
    cfg: Box<dyn Any>,
    methods: Methods,
}

impl SystemInfo {
    pub fn new<S: System>(cfg: S::Config) -> Self {
        Self {
            cfg: Box::new(cfg),
            methods: Methods {
                new: |ctx, cfg| Ok(Box::new(S::new(ctx, *cfg.downcast().unwrap())?)),
                pre_update: |this, ctx| S::pre_update(this.downcast_mut().unwrap(), ctx),
                post_update: |this, ctx| S::post_update(this.downcast_mut().unwrap(), ctx),
                pre_render: |this, ctx, draw| {
                    S::pre_render(this.downcast_mut().unwrap(), ctx, draw)
                },
                post_render: |this, ctx, draw| {
                    S::post_render(this.downcast_mut().unwrap(), ctx, draw)
                },
            },
        }
    }
}

pub(crate) struct DynSystem {
    system: Box<dyn Any>,
    methods: Methods,
}

impl DynSystem {
    pub fn new(ctx: &Context, info: SystemInfo) -> Result<Self, GameError> {
        Ok(Self {
            system: (info.methods.new)(ctx, info.cfg)?,
            methods: info.methods,
        })
    }

    #[inline]
    pub fn pre_update(&mut self, ctx: &Context) -> Result<(), GameError> {
        (self.methods.pre_update)(&mut self.system, ctx)
    }

    #[inline]
    pub fn post_update(&mut self, ctx: &Context) -> Result<(), GameError> {
        (self.methods.post_update)(&mut self.system, ctx)
    }

    #[inline]
    pub fn pre_render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        (self.methods.pre_render)(&mut self.system, ctx, draw)
    }

    #[inline]
    pub fn post_render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        (self.methods.post_render)(&mut self.system, ctx, draw)
    }
}

struct Methods {
    new: fn(ctx: &Context, cfg: Box<dyn Any>) -> Result<Box<dyn Any>, GameError>,
    pre_update: fn(this: &mut Box<dyn Any>, ctx: &Context) -> Result<(), GameError>,
    post_update: fn(this: &mut Box<dyn Any>, ctx: &Context) -> Result<(), GameError>,
    pre_render:
        fn(this: &mut Box<dyn Any>, ctx: &Context, draw: &mut Draw) -> Result<(), GameError>,
    post_render:
        fn(this: &mut Box<dyn Any>, ctx: &Context, draw: &mut Draw) -> Result<(), GameError>,
}
