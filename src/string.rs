use std::{slice, str, string::String as StdString};
use std::ops::{Deref, DerefMut};

use crate::error::{Error, Result};
use crate::ffi;
use crate::types::LuaRef;
use crate::util::{assert_stack, StackGuard};

/// Handle to an internal Lua string.
///
/// Unlike Rust strings, Lua strings may not be valid UTF-8.
#[derive(Clone, Debug)]
pub struct String<'lua>(pub(crate) LuaRef<'lua>);

impl<'lua> String<'lua> {
    /// Get a `&str` slice if the Lua string is valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rlua::{Lua, String, Result};
    /// # fn main() -> Result<()> {
    /// # Lua::new().context(|lua_context| {
    /// let globals = lua_context.globals();
    ///
    /// let version: String = globals.get("_VERSION")?;
    /// assert!(version.to_str().unwrap().contains("Lua"));
    ///
    /// let non_utf8: String = lua_context.load(r#"  "test\xff"  "#).eval()?;
    /// assert!(non_utf8.to_str().is_err());
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub fn to_str(&self) -> Result<&str> {
        str::from_utf8(self.as_bytes()).map_err(|e| Error::FromLuaConversionError {
            from: "string",
            to: "&str",
            message: Some(e.to_string()),
        })
    }

    /// Get the bytes that make up this string.
    ///
    /// The returned slice will not contain the terminating nul byte, but will contain any nul
    /// bytes embedded into the Lua string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rlua::{Lua, String, Result};
    /// # fn main() -> Result<()> {
    /// # Lua::new().context(|lua_context| -> Result<()> {
    /// let non_utf8: String = lua_context.load(r#"  "test\xff"  "#).eval()?;
    /// assert!(non_utf8.to_str().is_err());    // oh no :(
    /// assert_eq!(non_utf8.as_bytes(), &b"test\xff"[..]);
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        let nulled = self.as_bytes_with_nul();
        &nulled[..nulled.len() - 1]
    }

    /// Get the bytes that make up this string, including the trailing nul byte.
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        let lua = self.0.lua;
        unsafe {
            let _sg = StackGuard::new(lua.state);
            assert_stack(lua.state, 1);

            lua.push_ref(&self.0);
            rlua_debug_assert!(
                ffi::lua_type(lua.state, -1) == ffi::LUA_TSTRING,
                "string ref is not string type"
            );

            let mut size = 0;
            // This will not trigger a 'm' error, because the reference is guaranteed to be of
            // string type
            let data = ffi::lua_tolstring(lua.state, -1, &mut size);

            slice::from_raw_parts(data as *const u8, size + 1)
        }
    }
}

impl<'lua> AsRef<[u8]> for String<'lua> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// Lua strings are basically &[u8] slices, so implement PartialEq for anything resembling that.
//
// This makes our `String` comparable with `Vec<u8>`, `[u8]`, `&str`, `String` and `rlua::String`
// itself.
//
// The only downside is that this disallows a comparison with `Cow<str>`, as that only implements
// `AsRef<str>`, which collides with this impl. Requiring `AsRef<str>` would fix that, but limit us
// in other ways.
impl<'lua, T> PartialEq<T> for String<'lua>
where
    T: AsRef<[u8]>,
{
    fn eq(&self, other: &T) -> bool {
        self.as_bytes() == other.as_ref()
    }
}


/// A Rust type wrapper for byte-strings.
///
/// Because Lua strings can contain arbitrary data, not limited to just UTF-8, they can't always be
/// represented with the Rust `String` type. `ByteString` is a 'newtype' wrapper around a `Vec<u8>`
/// that indicates it should be converted to a string in Lua instead of an array of numbers.
#[derive(Clone, Debug)]
pub struct ByteString(pub (crate) Vec<u8>);

impl From<Vec<u8>> for ByteString {
    fn from(vec: Vec<u8>) -> Self {
        ByteString(vec)
    }
}

impl From<StdString> for ByteString {
    fn from(string: StdString) -> Self {
        ByteString(string.into())
    }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<T> PartialEq<T> for ByteString
where
    T: AsRef<[u8]>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}
