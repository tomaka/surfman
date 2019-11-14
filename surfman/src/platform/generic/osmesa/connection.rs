// surfman/surfman/src/platform/src/osmesa/connection.rs
//
//! Represents a connection to a display server.
//! 
//! For the OSMesa backend, this is a zero-sized types. OSMesa needs no connection, as it is a
//! CPU-based off-screen-only API.

use crate::Error;
use super::device::{Adapter, Device};
use super::surface::NativeWidget;

#[cfg(feature = "sm-winit")]
use winit::Window;

/// A no-op connection.
#[derive(Clone)]
pub struct Connection;

impl Connection {
    /// Connects to the default display.
    #[inline]
    pub fn new() -> Result<Connection, Error> {
        Ok(Connection)
    }

    /// Returns the "best" adapter on this system, preferring high-performance hardware adapters.
    /// 
    /// This is an alias for `Connection::create_hardware_adapter()`.
    #[inline]
    pub fn create_adapter(&self) -> Result<Adapter, Error> {
        self.create_hardware_adapter()
    }

    /// Returns the "best" adapter on this system, preferring high-performance hardware adapters.
    /// 
    /// On the OSMesa backend, this returns a software adapter.
    #[inline]
    pub fn create_hardware_adapter(&self) -> Result<Adapter, Error> {
        self.create_software_adapter()
    }

    /// Returns the "best" adapter on this system, preferring low-power hardware adapters.
    /// 
    /// On the OSMesa backend, this returns a software adapter.
    #[inline]
    pub fn create_low_power_adapter(&self) -> Result<Adapter, Error> {
        self.create_software_adapter()
    }

    /// Returns the "best" adapter on this system, preferring software adapters.
    #[inline]
    pub fn create_software_adapter(&self) -> Result<Adapter, Error> {
        Ok(Adapter)
    }

    /// Opens the hardware device corresponding to the given adapter.
    /// 
    /// Device handles are local to a single thread.
    #[inline]
    pub fn create_device(&self, _: &Adapter) -> Result<Device, Error> {
        Device::new()
    }

    /// Opens the display connection corresponding to the given `winit` window.
    #[inline]
    #[cfg(feature = "sm-winit")]
    pub fn from_winit_window(_: &Window) -> Result<Connection, Error> {
        Err(Error::IncompatibleNativeWidget)
    }

    /// Creates a native widget type from the given `winit` window.
    /// 
    /// This type can be later used to create surfaces that render to the window.
    #[inline]
    #[cfg(feature = "sm-winit")]
    pub fn create_native_widget_from_winit_window(&self, _: &Window)
                                                  -> Result<NativeWidget, Error> {
        Err(Error::IncompatibleNativeWidget)
    }
}
