// surfman/src/platform/windows/wgl/context.rs
//
//! Wrapper for WGL contexts on Windows.

use super::device::Device;
use super::surface::Surface;
use crate::context::{self, CREATE_CONTEXT_MUTEX};
use crate::surface::Framebuffer;
use crate::{ContextAttributeFlags, ContextAttributes, ContextID, Error, GLVersion};
use crate::{SurfaceInfo, WindowingApiError};

use crate::gl;
use crate::gl::types::{GLenum, GLint, GLuint};
use crate::Gl;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::thread;

/// Information needed to create a context. Some APIs call this a "config" or a "pixel format".
///
/// These are local to a device.
#[derive(Clone)]
pub struct ContextDescriptor {
}

/// Represents an OpenGL rendering context.
///
/// A context allows you to issue rendering commands to a surface. When initially created, a
/// context has no attached surface, so rendering commands will fail or be ignored. Typically, you
/// attach a surface to the context before rendering.
///
/// Contexts take ownership of the surfaces attached to them. In order to mutate a surface in any
/// way other than rendering to it (e.g. presenting it to a window, which causes a buffer swap), it
/// must first be detached from its context. Each surface is associated with a single context upon
/// creation and may not be rendered to from any other context. However, you can wrap a surface in
/// a surface texture, which allows the surface to be read from another context.
///
/// OpenGL objects may not be shared across contexts directly, but surface textures effectively
/// allow for sharing of texture data. Contexts are local to a single thread and device.
///
/// A context must be explicitly destroyed with `destroy_context()`, or a panic will occur.
pub struct Context {
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ContextStatus {
    Owned,
    Referenced,
    Destroyed,
}

/// Wrapper for a WGL `HGLRC`.
#[derive(Clone)]
pub struct NativeContext;

impl Device {
    /// Creates a context descriptor with the given attributes.
    ///
    /// Context descriptors are local to this device.
    #[allow(non_snake_case)]
    pub fn create_context_descriptor(
        &self,
        attributes: &ContextAttributes,
    ) -> Result<ContextDescriptor, Error> {
        unimplemented!()
    }

    /// Creates a new OpenGL context.
    ///
    /// The context initially has no surface attached. Until a surface is bound to it, rendering
    /// commands will fail or have no effect.
    #[allow(non_snake_case)]
    pub fn create_context(
        &mut self,
        descriptor: &ContextDescriptor,
        share_with: Option<&Context>,
    ) -> Result<Context, Error> {
        unimplemented!()
    }

    /// Wraps an `HGLRC` in a `surfman` context and returns it.
    ///
    /// The `HGLRC` is not retained, as there is no way to do this in the Win32 API. Therefore, it
    /// is the caller's responsibility to make sure the OpenGL context is not destroyed before this
    /// `Context` is.
    pub unsafe fn create_context_from_native_context(
        &self,
        native_context: NativeContext,
    ) -> Result<Context, Error> {
        unimplemented!()
    }

    /// Destroys a context.
    ///
    /// The context must have been created on this device.
    pub fn destroy_context(&self, context: &mut Context) -> Result<(), Error> {
        unimplemented!()
    }

    /// Returns the descriptor that this context was created with.
    pub fn context_descriptor(&self, context: &Context) -> ContextDescriptor {
        unimplemented!()
    }

    /// Returns the attributes that the context descriptor was created with.
    #[allow(non_snake_case)]
    pub fn context_descriptor_attributes(
        &self,
        context_descriptor: &ContextDescriptor,
    ) -> ContextAttributes {
        unimplemented!()
    }

    /// Makes the context the current OpenGL context for this thread.
    ///
    /// After calling this function, it is valid to use OpenGL rendering commands.
    pub fn make_context_current(&self, context: &Context) -> Result<(), Error> {
        unimplemented!()
    }

    /// Removes the current OpenGL context from this thread.
    ///
    /// After calling this function, OpenGL rendering commands will fail until a new context is
    /// made current.
    #[inline]
    pub fn make_no_context_current(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Fetches the address of an OpenGL function associated with this context.
    ///
    /// OpenGL functions are local to a context. You should not use OpenGL functions on one context
    /// with any other context.
    ///
    /// This method is typically used with a function like `gl::load_with()` from the `gl` crate to
    /// load OpenGL function pointers.
    #[inline]
    pub fn get_proc_address(&self, _: &Context, symbol_name: &str) -> *const c_void {
        unimplemented!()
    }

    #[inline]
    fn context_is_current(&self, context: &Context) -> bool {
        unimplemented!()
    }

    /// Attaches a surface to a context for rendering.
    ///
    /// This function takes ownership of the surface. The surface must have been created with this
    /// context, or an `IncompatibleSurface` error is returned.
    ///
    /// If this function is called with a surface already bound, a `SurfaceAlreadyBound` error is
    /// returned. To avoid this error, first unbind the existing surface with
    /// `unbind_surface_from_context`.
    ///
    /// If an error is returned, the surface is returned alongside it.
    pub fn bind_surface_to_context(
        &self,
        context: &mut Context,
        surface: Surface,
    ) -> Result<(), (Error, Surface)> {
        unimplemented!()
    }

    /// Removes and returns any attached surface from this context.
    ///
    /// Any pending OpenGL commands targeting this surface will be automatically flushed, so the
    /// surface is safe to read from immediately when this function returns.
    pub fn unbind_surface_from_context(
        &self,
        context: &mut Context,
    ) -> Result<Option<Surface>, Error> {
        unimplemented!()
    }

    /// Returns a unique ID representing a context.
    ///
    /// This ID is unique to all currently-allocated contexts. If you destroy a context and create
    /// a new one, the new context might have the same ID as the destroyed one.
    #[inline]
    pub fn context_id(&self, context: &Context) -> ContextID {
        unimplemented!()
    }

    /// Returns various information about the surface attached to a context.
    ///
    /// This includes, most notably, the OpenGL framebuffer object needed to render to the surface.
    pub fn context_surface_info(&self, context: &Context) -> Result<Option<SurfaceInfo>, Error> {
        unimplemented!()
    }

    /// Given a context, returns its underlying `HGLRC`.
    #[inline]
    pub fn native_context(&self, context: &Context) -> NativeContext {
        unimplemented!()
    }
}

impl NativeContext {
    /// Returns the current context, if there is one.
    ///
    /// If there is not a native context, this returns a `NoCurrentContext` error.
    #[inline]
    pub fn current() -> Result<NativeContext, Error> {
        unimplemented!()
    }
}
