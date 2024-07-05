//! Contains definitions from `lualib.h`.

use std::os::raw::c_int;

use super::lua::lua_State;

#[cfg(not(feature = "factorio"))]
pub const LUA_COLIBNAME: &str = "coroutine";
pub const LUA_TABLIBNAME: &str = "table";
#[cfg(not(feature = "factorio"))]
pub const LUA_IOLIBNAME: &str = "io";
#[cfg(not(feature = "factorio"))]
pub const LUA_OSLIBNAME: &str = "os";
pub const LUA_STRLIBNAME: &str = "string";
pub const LUA_BITLIBNAME: &str = "bit32";
pub const LUA_MATHLIBNAME: &str = "math";
pub const LUA_DBLIBNAME: &str = "debug";
#[cfg(not(feature = "factorio"))]
pub const LUA_LOADLIBNAME: &str = "package";

#[cfg_attr(all(windows, raw_dylib), link(name = "lua52", kind = "raw-dylib"))]
extern "C-unwind" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    #[cfg(not(feature = "factorio"))]
    pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    #[cfg(not(feature = "factorio"))]
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    #[cfg(not(feature = "factorio"))]
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    #[cfg(not(feature = "factorio"))]
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    #[cfg(feature = "factorio")]
    pub fn luaopen_fulldebug(L: *mut lua_State) -> c_int;
    #[cfg(feature = "factorio")]
    pub fn luaopen_partialdebug(L: *mut lua_State) -> c_int;
    #[cfg(not(feature = "factorio"))]
    pub fn luaopen_package(L: *mut lua_State) -> c_int;

    // open all builtin libraries
    #[cfg(not(feature = "factorio"))]
    pub fn luaL_openlibs(L: *mut lua_State);
    #[cfg(feature = "factorio")]
    pub fn luaL_openlibs(L: *mut lua_State, load_full_debug: c_int);
}
