use crate::{Device, PayloadStruct};
use failure::format_err;
use failure::Error;

fn check_id(payload: &PayloadStruct, ref_id: u8) -> Result<(), Error> {
    if payload.id != ref_id {
        return Err(format_err!(
            "id should be {}, but it is {}",
            ref_id,
            payload.id
        ));
    }
    Ok(())
}

pub struct DobotClient<T: Device> {
    device: T,
}

impl<T> DobotClient<T>
where
    T: Device,
{
    pub fn new(device: T) -> Self {
        Self { device }
    }

    fn write_params(&mut self, id: u8, params: Vec<u8>) -> Result<(), Error> {
        let p = PayloadStruct::with_id(id).set_write().set_params(params);
        let ret = self.device.send(p)?;
        check_id(&ret, id)
    }

    fn write_queued_params(&mut self, id: u8, params: Vec<u8>) -> Result<u64, Error> {
        let p = PayloadStruct::with_id(id)
            .set_write()
            .set_params(params)
            .set_queued();
        let ret = self.device.send(p)?;
        check_id(&ret, id)?;
        let mut u = U64Union { val: 0 };
        unsafe {
            u.bytes.copy_from_slice(&ret.params);
            Ok(u.val)
        }
    }

    fn read_params(&mut self, id: u8) -> Result<Vec<u8>, Error> {
        let p = PayloadStruct::with_id(id);
        let ret = self.device.send(p)?;
        check_id(&ret, id)?;
        Ok(ret.params)
    }

    pub fn get_device_sn(&mut self) -> Result<String, Error> {
        Ok(String::from_utf8(self.read_params(0)?)?)
    }

    pub fn get_alarm_state(&mut self) -> Result<Vec<u8>, Error> {
        self.read_params(20)
    }

    pub fn clear_all_alarm_state(&mut self) -> Result<(), Error> {
        self.write_params(20, vec![])
    }

    pub fn get_pose(&mut self) -> Result<Pose, Error> {
        let p = self.read_params(10)?;
        let mut pose_union = PoseUnion { bytes: [0; 32] };
        let pose = unsafe {
            pose_union.bytes.copy_from_slice(&p);
            pose_union.poses
        };
        Ok(pose)
    }

    pub fn get_jog_joint_params(&mut self) -> Result<JogJointParams, Error> {
        let p = self.read_params(70)?;
        let mut params_union = JogJointParamsUnion { bytes: [0; 32] };
        let params = unsafe {
            params_union.bytes.copy_from_slice(&p);
            params_union.jog_joint_params
        };
        Ok(params)
    }

    pub fn set_jog_joint_params(&mut self, params: JogJointParams) -> Result<(), Error> {
        let params_union = JogJointParamsUnion {
            jog_joint_params: params,
        };
        self.write_params(70, unsafe { params_union.bytes.to_vec() })
    }

    pub fn get_jog_common_params(&mut self) -> Result<JogCommonParams, Error> {
        let p = self.read_params(72)?;
        let mut params_union = JogCommonParamsUnion { bytes: [0; 8] };
        let params = unsafe {
            params_union.bytes.copy_from_slice(&p);
            params_union.jog_common_params
        };
        Ok(params)
    }

    pub fn set_jog_common_params(&mut self, params: JogCommonParams) -> Result<(), Error> {
        let params_union = JogCommonParamsUnion {
            jog_common_params: params,
        };
        self.write_params(72, unsafe { params_union.bytes.to_vec() })
    }

    pub fn set_jog_command(&mut self, mode: JogCommandType, cmd: JogCommand) -> Result<(), Error> {
        self.write_params(73, vec![mode as u8, cmd as u8])
    }

    pub fn get_ptp_joint_params(&mut self) -> Result<PtpJointParams, Error> {
        let p = self.read_params(80)?;
        let mut params_union = PtpJointParamsUnion { bytes: [0; 32] };
        let params = unsafe {
            params_union.bytes.copy_from_slice(&p);
            params_union.ptp_joint_params
        };
        Ok(params)
    }

    pub fn set_ptp_joint_params(&mut self, params: PtpJointParams) -> Result<(), Error> {
        let params_union = PtpJointParamsUnion {
            ptp_joint_params: params,
        };
        self.write_params(80, unsafe { params_union.bytes.to_vec() })
    }

    pub fn get_ptp_coordinate_params(&mut self) -> Result<PtpCoordinateParams, Error> {
        let p = self.read_params(81)?;
        let mut params_union = PtpCoordinateParamsUnion { bytes: [0; 16] };
        let params = unsafe {
            params_union.bytes.copy_from_slice(&p);
            params_union.ptp_coordinate_params
        };
        Ok(params)
    }

    pub fn set_ptp_coordinate_params(&mut self, params: PtpCoordinateParams) -> Result<(), Error> {
        let params_union = PtpCoordinateParamsUnion {
            ptp_coordinate_params: params,
        };
        self.write_params(81, unsafe { params_union.bytes.to_vec() })
    }

    pub fn get_ptp_jump_params(&mut self) -> Result<PtpJumpParams, Error> {
        let p = self.read_params(82)?;
        let mut params_union = PtpJumpParamsUnion { bytes: [0; 12] };
        let params = unsafe {
            params_union.bytes.copy_from_slice(&p);
            params_union.ptp_jump_params
        };
        Ok(params)
    }

    pub fn set_ptp_jump_params(&mut self, params: PtpJumpParams) -> Result<(), Error> {
        let params_union = PtpJumpParamsUnion {
            ptp_jump_params: params,
        };
        self.write_params(82, unsafe { params_union.bytes.to_vec() })
    }

    pub fn get_ptp_common_params(&mut self) -> Result<PtpCommonParams, Error> {
        let p = self.read_params(83)?;
        let mut params_union = PtpCommonParamsUnion { bytes: [0; 8] };
        let params = unsafe {
            params_union.bytes.copy_from_slice(&p);
            params_union.ptp_common_params
        };
        Ok(params)
    }

    pub fn set_ptp_common_params(&mut self, params: PtpCommonParams) -> Result<(), Error> {
        let params_union = PtpCommonParamsUnion {
            ptp_common_params: params,
        };
        self.write_params(83, unsafe { params_union.bytes.to_vec() })
    }

    pub fn set_ptp_command(&mut self, command: PtpCommand) -> Result<(), Error> {
        let command_union = PtpCommandUnion {
            ptp_command: command,
        };
        self.write_params(84, unsafe { command_union.bytes.to_vec() })
    }

    pub fn set_ptp_command_queued(&mut self, command: PtpCommand) -> Result<u64, Error> {
        let command_union = PtpCommandUnion {
            ptp_command: command,
        };
        self.write_queued_params(84, unsafe { command_union.bytes.to_vec() })
    }

    // address = (1 ~ 22), air pump is connected to 18.
    pub fn set_iodo(&mut self, address: u8, level: IoLevel) -> Result<(), Error> {
        self.write_params(131, vec![address, level as u8])
    }

    pub fn set_iodo_queued(&mut self, address: u8, level: IoLevel) -> Result<u64, Error> {
        self.write_queued_params(131, vec![address, level as u8])
    }

    // wait
    pub fn set_wait_command(&mut self, wait_ms: u32) -> Result<(), Error> {
        let u = U32Union { val: wait_ms };
        self.write_params(110, unsafe { u.bytes.to_vec() })
    }

    pub fn set_wait_command_queued(&mut self, wait_ms: u32) -> Result<u64, Error> {
        let u = U32Union { val: wait_ms };
        self.write_queued_params(110, unsafe { u.bytes.to_vec() })
    }

    pub fn set_arm_orientation(&mut self, l_r: ArmOrientation) -> Result<(), Error> {
        self.write_params(50, vec![l_r as u8])
    }

    pub fn set_arm_orientation_queued(&mut self, l_r: ArmOrientation) -> Result<u64, Error> {
        self.write_queued_params(50, vec![l_r as u8])
    }

    pub fn set_queued_command_start_exec(&mut self) -> Result<(), Error> {
        self.write_params(240, vec![])
    }

    pub fn set_queued_command_stop_exec(&mut self) -> Result<(), Error> {
        self.write_params(241, vec![])
    }

    pub fn set_queued_command_force_stop_exec(&mut self) -> Result<(), Error> {
        self.write_params(242, vec![])
    }

    pub fn set_queued_command_clear(&mut self) -> Result<(), Error> {
        self.write_params(245, vec![])
    }

    pub fn get_queued_command_current_index(&mut self) -> Result<u64, Error> {
        let params = self.read_params(246)?;
        let mut u = U64Union { val: 0 };
        unsafe {
            u.bytes.copy_from_slice(&params);
            Ok(u.val)
        }
    }

    pub fn get_queued_command_left_space(&mut self) -> Result<u32, Error> {
        let params = self.read_params(247)?;
        let mut u = U32Union { val: 0 };
        unsafe {
            u.bytes.copy_from_slice(&params);
            Ok(u.val)
        }
    }

    pub fn get_arm_orientation(&mut self) -> Result<ArmOrientation, Error> {
        let lr = self.read_params(50)?[0];
        Ok(if lr == 0 {
            ArmOrientation::Lefty
        } else {
            ArmOrientation::Righty
        })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum IoLevel {
    Low,
    High,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ArmOrientation {
    Lefty,
    Righty,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum JogCommandType {
    Cartesian,
    Joint,
}

#[repr(u8)]
pub enum JogCommand {
    Idel,
    ApDown,
    AnDown,
    BpDown,
    BnDown,
    CpDown,
    CnDown,
    DpDown,
    DnDown,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub joint_angles: [f32; 4],
}

//#[repr(C)]
union PoseUnion {
    poses: Pose,
    bytes: [u8; 32],
}

//#[repr(C)]
union U64Union {
    val: u64,
    pub bytes: [u8; 8],
}

//#[repr(C)]
union U32Union {
    val: u32,
    pub bytes: [u8; 4],
}

/// JOG
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct JogJointParams {
    pub velocity: [f32; 4],
    pub acceleration: [f32; 4],
}

//#[repr(C)]
union JogJointParamsUnion {
    jog_joint_params: JogJointParams,
    bytes: [u8; 32],
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct JogCommonParams {
    pub velocity_ratio: f32,
    pub acceleration_ratio: f32,
}

//#[repr(C)]
union JogCommonParamsUnion {
    jog_common_params: JogCommonParams,
    bytes: [u8; 8],
}

/// PTP
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct PtpJointParams {
    pub velocity: [f32; 4],
    pub acceleration: [f32; 4],
}

#[repr(packed)]
union PtpJointParamsUnion {
    ptp_joint_params: PtpJointParams,
    bytes: [u8; 32],
}

//#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PtpCoordinateParams {
    pub xyz_velocity: f32,
    pub r_velocity: f32,
    pub xyz_acceleration: f32,
    pub r_acceleration: f32,
}

//#[repr(C)]
union PtpCoordinateParamsUnion {
    ptp_coordinate_params: PtpCoordinateParams,
    bytes: [u8; 16],
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct PtpJumpParams {
    pub jump_height: f32,
    pub z_limit: f32,
    pub dummy: u32,
}

#[repr(packed)]
union PtpJumpParamsUnion {
    ptp_jump_params: PtpJumpParams,
    bytes: [u8; 12],
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct PtpCommonParams {
    pub velocity_ratio: f32,
    pub acceleration_ratio: f32,
}

//#[repr(C)]
union PtpCommonParamsUnion {
    ptp_common_params: PtpCommonParams,
    bytes: [u8; 8],
}

//#[repr(C)]
union PtpCommandUnion {
    ptp_command: PtpCommand,
    bytes: [u8; 17],
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct PtpCommand {
    pub ptp_mode: PtpMode,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum PtpMode {
    JumpXyz,     // JUMP mode, (x,y,z,r) is the target point in Cartesian coordinate system
    MovjXyz,     // MOVJ mode, (x,y,z,r) is the target point in Cartesian coordinate system
    MovlXyz,     // MOVL mode, (x,y,z,r) is the target point in Cartesian coordinate system
    JumpAngle,   // JUMP mode, (x,y,z,r) is the target point in Joint coordinate system
    MovjAngle,   // MOVJ mode, (x,y,z,r) is the target point in Joint coordinate system
    MovlAngle,   // MOVL mode, (x,y,z,r) is the target point in Joint coordinate system
    MovjInc,     // MOVJ mode, (x,y,z,r) is the angle increment in Joint coordinate system
    MovlInc, // MOVL mode, (x,y,z,r) is the Cartesian coordinate increment in Joint coordinate system
    MovjXyzInc, // MOVJ mode, (x,y,z,r) is the Cartesian coordinate increment in Cartesian coordinate system
    JumpMovlXyz, // JUMP mode, (x,y,z,r) is the Cartesian coordinate increment in Cartesian coordinate
}
