use super::{Context, Game, GameError};
use crate::gfx::Draw;
use fey_lua::TempTypes;
use mlua::prelude::LuaResult;
use mlua::{Function, Lua, Table, Value};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct LuaGame<G: Game> {
    lua: Lua,
    default_globals: HashSet<String>,
    default_modules: HashSet<String>,
    main: LuaResult<LuaMain>,
    call_lua_init: bool,
    game: G,
}

pub struct LuaConfig<G: Game> {
    pub cfg: G::Config,
    pub prefix_modules: bool,
}

impl<G: Game> Game for LuaGame<G> {
    type Config = LuaConfig<G>;

    fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        let lua = ctx.lua.upgrade();
        let game = G::new(ctx, cfg.cfg)?;

        // add context to lua
        assert!(
            lua.set_app_data(ctx.clone()).is_none(),
            "context was already added to Lua (bad)"
        );

        // make sure our lua entry point exists
        let main_path = PathBuf::from("lua/Main.lua");
        assert!(
            main_path.exists() && main_path.is_file(),
            "lua entry point not found at: {main_path:?}"
        );

        // preload all scripts in the root lua folder
        fn read_dir(lua: &Lua, dir: PathBuf, prefix: Option<String>) -> Result<(), GameError> {
            for entry in std::fs::read_dir(&dir)?.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(OsStr::to_str) {
                        let prefix = prefix.as_ref().map(|prefix| format!("{prefix}{name}."));
                        read_dir(lua, dir.join(name), prefix)?;
                    }
                } else if path.is_file() && path.extension().is_some_and(|ext| ext == "lua") {
                    if let Some(name) = path.file_stem().and_then(OsStr::to_str) {
                        let path = path.clone();
                        let file_name = path.to_string_lossy().to_string();
                        //let mod_name = format!("{prefix}{name}");
                        let mod_name = match prefix.as_ref() {
                            Some(prefix) => format!("{prefix}{name}"),
                            None => name.to_string(),
                        };
                        lua.preload_module(
                            &mod_name,
                            lua.create_function(move |lua, _: ()| {
                                let code = std::fs::read_to_string(&path)?;
                                lua.load(code)
                                    .set_name(format!("@{file_name}"))
                                    .eval::<Value>()
                            })?,
                        )?;
                    }
                }
            }
            Ok(())
        }
        read_dir(&lua, "lua".into(), cfg.prefix_modules.then(String::new))?;

        // get a list of all the default globals
        let default_globals = lua
            .globals()
            .pairs::<String, Value>()
            .map(|p| p.unwrap().0)
            .collect();

        // get a list of all the default modules
        let default_modules = lua
            .globals()
            .get::<Table>("package")
            .unwrap()
            .get::<Table>("loaded")
            .unwrap()
            .pairs::<String, Value>()
            .map(|p| p.unwrap().0)
            .collect();

        // load up the entry point
        let main = LuaMain::load(&lua, &default_globals, &default_modules);
        let call_lua_init = if let Err(err) = &main {
            println!("{err}");
            false
        } else {
            true
        };

        Ok(Self {
            lua,
            default_globals,
            default_modules,
            main,
            call_lua_init,
            game,
        })
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        self.game.update(ctx)?;

        // reload the lua if requested
        if ctx.reload_lua.take() {
            self.main = LuaMain::load(&self.lua, &self.default_globals, &self.default_modules);
            self.call_lua_init = if let Err(err) = &self.main {
                println!("{err}");
                false
            } else {
                true
            };
            self.lua.gc_collect()?;
            self.lua.gc_collect()?;
        }

        // call Main:init() when requested
        if self.call_lua_init {
            self.call_lua_init = false;

            if let Ok(Err(err)) = self.main.as_ref().map(|main| main.init()) {
                println!("{err}");
                self.main = Err(err);
            }
        }

        // call Main:update()
        if let Ok(Err(err)) = self.main.as_ref().map(|main| main.update()) {
            println!("{err}");
            self.main = Err(err);
        }

        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        // call Main:render()
        if let Ok(Err(err)) = self.main.as_ref().map(|main| main.render(&self.lua, draw)) {
            println!("{err}");
            self.main = Err(err);
        }

        // clear all single-frame temp types
        self.lua.app_data_mut::<TempTypes>().unwrap().clear_frame();

        self.game.render(ctx, draw)?;

        Ok(())
    }
}

struct LuaMain {
    module: Table,
    init_fn: Function,
    update_fn: Function,
    render_fn: Function,
}

impl LuaMain {
    pub fn load(
        lua: &Lua,
        default_globals: &HashSet<String>,
        default_modules: &HashSet<String>,
    ) -> LuaResult<Self> {
        // unload non-default globals and modules
        {
            let g = lua.globals();
            let remove: Vec<String> = g
                .pairs::<String, Value>()
                .map(|p| p.unwrap().0)
                .filter(|k| !default_globals.contains(k))
                .collect();
            for k in remove {
                g.set(k, Value::Nil)?;
            }
            let loaded = g.get::<Table>("package")?.get::<Table>("loaded")?;
            let remove: Vec<String> = loaded
                .pairs::<String, Value>()
                .map(|p| p.unwrap().0)
                .filter(|k| !default_modules.contains(k))
                .collect();
            for k in remove {
                loaded.set(k, Value::Nil)?;
            }
        }

        // load up the lua entry point
        let module = lua
            .globals()
            .get::<Function>("require")
            .unwrap()
            .call::<Table>("Main")?;
        let init_fn = module.get("init")?;
        let update_fn = module.get("update")?;
        let render_fn = module.get("render")?;

        // run the garbage collector
        lua.gc_collect()?;
        lua.gc_collect()?;

        Ok(Self {
            module,
            init_fn,
            update_fn,
            render_fn,
        })
    }

    #[inline]
    fn init(&self) -> LuaResult<()> {
        self.init_fn.call(self.module.clone())
    }

    #[inline]
    fn update(&self) -> LuaResult<()> {
        self.update_fn.call(self.module.clone())
    }

    #[inline]
    fn render(&self, lua: &Lua, draw: &mut Draw) -> LuaResult<()> {
        let draw: *mut Draw = draw;
        assert!(lua.set_app_data(draw).is_none());
        self.render_fn.call::<()>(self.module.clone())?;
        assert!(lua.remove_app_data::<*mut Draw>().is_some());
        Ok(())
    }
}
