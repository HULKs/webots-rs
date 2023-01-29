use std::slice::from_raw_parts;

use thiserror::Error;
use webots_bindings::{
    wb_device_get_node_type, wb_receiver_disable, wb_receiver_enable, wb_receiver_get_channel,
    wb_receiver_get_data, wb_receiver_get_data_size, wb_receiver_get_emitter_direction,
    wb_receiver_get_queue_length, wb_receiver_get_sampling_period, wb_receiver_get_signal_strength,
    wb_receiver_next_packet, wb_receiver_set_channel, WbDeviceTag, WbNodeType_WB_NODE_RECEIVER,
};

#[derive(Debug, Error)]
pub enum ReceiverError {
    #[error("failed to get emitter direction: emitter direction data is NULL")]
    EmitterDirectionIsNull,
}

pub struct Receiver(WbDeviceTag);

impl Receiver {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_RECEIVER, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_receiver_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_receiver_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_receiver_get_sampling_period(self.0) }
    }

    pub fn get_next_packet(&self) -> Result<Option<Packet>, ReceiverError> {
        let queue_length = unsafe { wb_receiver_get_queue_length(self.0) };
        if queue_length > 0 {
            let data = unsafe {
                from_raw_parts(
                    wb_receiver_get_data(self.0) as *const u8,
                    wb_receiver_get_data_size(self.0) as usize,
                )
            }
            .to_vec();
            let signal_strength = unsafe { wb_receiver_get_signal_strength(self.0) };
            let emitter_direction = unsafe {
                let emitter_direction = wb_receiver_get_emitter_direction(self.0);
                if emitter_direction.is_null() {
                    return Err(ReceiverError::EmitterDirectionIsNull);
                }
                [
                    *emitter_direction.offset(0),
                    *emitter_direction.offset(1),
                    *emitter_direction.offset(2),
                ]
            };

            let packet = Packet {
                data,
                signal_strength,
                emitter_direction,
            };

            unsafe { wb_receiver_next_packet(self.0) };

            Ok(Some(packet))
        } else {
            Ok(None)
        }
    }

    pub fn set_channel(&self, channel: i32) {
        unsafe { wb_receiver_set_channel(self.0, channel) }
    }

    pub fn get_channel(&self) -> i32 {
        unsafe { wb_receiver_get_channel(self.0) }
    }
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub data: Vec<u8>,
    pub signal_strength: f64,
    pub emitter_direction: [f64; 3],
}
