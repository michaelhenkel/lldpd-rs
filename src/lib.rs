pub mod bindings;
use crate::bindings::*;
use std::{ffi::CStr, ptr};

const LLDPCTL_K_CHASSIS_ID: u32 = 1802;
const LLDPCTL_K_INTERFACE_NAME: u32 = 1000;
const LLDPCTL_K_PORT_NEIGHBORS: u32 = 1200;

pub fn get_remote_chassis_id(interface_input: &str) -> Option<String>{
    unsafe{
        let handle = lldpctl_new(None, None, ptr::null_mut());
        if handle.is_null() {
            return None;
        }
        let interfaces = lldpctl_get_interfaces(handle);
        if !interfaces.is_null() {
            let mut iterator = lldpctl_atom_iter(interfaces);
            while !iterator.is_null() {
                let interface_atom = lldpctl_atom_iter_value(interfaces, iterator);
                let interface_name_ptr = lldpctl_atom_get_str(interface_atom, LLDPCTL_K_INTERFACE_NAME);
                if !interface_name_ptr.is_null() {
                    let interface_name = CStr::from_ptr(interface_name_ptr)
                        .to_str()
                        .unwrap_or("Invalid UTF-8");
                    if interface_name == interface_input {
                        let port_atom = lldpctl_get_port(interface_atom);
                        if !port_atom.is_null() {
                            let neighbors = lldpctl_atom_get(port_atom, LLDPCTL_K_PORT_NEIGHBORS);
                            if !neighbors.is_null() {
                                let mut neighbor_iterator = lldpctl_atom_iter(neighbors);
                                while !neighbor_iterator.is_null() {
                                    let neighbor_atom =
                                        lldpctl_atom_iter_value(neighbors, neighbor_iterator);
                                    let chassis_id_ptr =
                                        lldpctl_atom_get_str(neighbor_atom, LLDPCTL_K_CHASSIS_ID);
                                    if !chassis_id_ptr.is_null() {
                                        let chassis_id = CStr::from_ptr(chassis_id_ptr)
                                            .to_str()
                                            .unwrap_or("Invalid UTF-8");
                                        return Some(chassis_id.to_string());
                                    }
                                    neighbor_iterator =
                                        lldpctl_atom_iter_next(neighbors, neighbor_iterator);
                                }
                            }
                        }
                    }
                }
                iterator = lldpctl_atom_iter_next(interfaces, iterator);
            }
        }
        lldpctl_release(handle);
    }
    None
}