// This file was generated by gir (651da6e) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PlayerSignalDispatcher(Object<ffi::GstPlayerSignalDispatcher>);

    match fn {
        get_type => || ffi::gst_player_signal_dispatcher_get_type(),
    }
}

unsafe impl Send for PlayerSignalDispatcher {}
unsafe impl Sync for PlayerSignalDispatcher {}

pub trait PlayerSignalDispatcherExt {}

impl<O: IsA<PlayerSignalDispatcher>> PlayerSignalDispatcherExt for O {}
