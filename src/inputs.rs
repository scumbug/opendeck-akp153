use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::KEY_COUNT;

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::info!("Processing input: {}, {}", input, state);

    match input as usize {
        (0..=KEY_COUNT) => read_button_press(input, state),
        _ => Err(MirajazzError::BadData),
    }
}

fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..KEY_COUNT {
        bools.push(states[i + 1] != 0);
    }

    bools
}

/// Converts opendeck key index to device LCD id
///
/// LCD ids are 0-based here, mirajazz adds 1 to them when sending to the device.
/// Device LCD ids are laid out as: row 0 = 0x0b-0x0f, row 1 = 0x06-0x0a, row 2 = 0x01-0x05
pub fn opendeck_to_device(key: u8) -> u8 {
    if key < KEY_COUNT as u8 {
        [10, 11, 12, 13, 14, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4][key as usize]
    } else {
        key
    }
}

/// Converts device button id to opendeck key index
///
/// Button ids 1..15 are laid out row-major, so the opendeck key is the id minus 1.
pub fn device_to_opendeck(id: usize) -> usize {
    // We have to subtract 1 from key index reported by device, because ids are 1-based
    let key = id - 1;

    if key < KEY_COUNT {
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14][key]
    } else {
        key
    }
}

fn read_button_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]);

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    let pressed_index: usize = device_to_opendeck(input as usize);

    // `device_to_opendeck` is 0-based, so add 1 to index into the 1-based state list
    button_states[pressed_index + 1] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}
