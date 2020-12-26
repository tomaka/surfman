// surfman/src/platform/windows/wgl/device.rs
//
//! An implementation of the GPU device for Windows using the WGL API.

use super::connection::Connection;
use crate::{Error, GLApi};

use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

pub(crate) const HIDDEN_WINDOW_SIZE: c_int = 16;

/// Represents a hardware display adapter that can be used for rendering (including the CPU).
///
/// Adapters can be sent between threads. To render with an adapter, open a thread-local `Device`.
#[derive(Clone, Debug)]
pub enum Adapter {
    #[doc(hidden)]
    HighPerformance,
    #[doc(hidden)]
    LowPower,
}

/// A thread-local handle to a device.
///
/// Devices contain most of the relevant surface management methods.
#[allow(dead_code)]
pub struct Device {
}

/// Wraps a Direct3D 11 device and its associated GL/DX interop device.
#[derive(Clone)]
pub struct NativeDevice {
}

impl Adapter {
    pub(crate) fn set_exported_variables(&self) {
        unimplemented!()
    }
}

impl Device {
    pub(crate) fn new(adapter: &Adapter) -> Result<Device, Error> {
        unimplemented!()
    }

    pub(crate) fn from_native_device(native_device: NativeDevice) -> Result<Device, Error> {
        unimplemented!()
    }

    /// Returns the associated Direct3D 11 device and GL/DX interop device handle.
    ///
    /// The reference count on the D3D 11 device is increased before returning.
    #[inline]
    pub fn native_device(&self) -> NativeDevice {
        unimplemented!()
    }

    /// Returns the display server connection that this device was created with.
    #[inline]
    pub fn connection(&self) -> Connection {
        Connection
    }

    /// Returns the adapter that this device was created with.
    #[inline]
    pub fn adapter(&self) -> Adapter {
        unimplemented!()
    }

    /// Returns the OpenGL API flavor that this device supports (OpenGL or OpenGL ES).
    #[inline]
    pub fn gl_api(&self) -> GLApi {
        GLApi::GL
    }
}
