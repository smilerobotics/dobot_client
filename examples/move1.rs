use dobot_client::*;

fn hold<T>(dobot: &mut DobotClient<T>) -> Result<u64, failure::Error>
where
    T: Device,
{
    dobot.set_iodo_queued(17, IoLevel::Low)?;
    dobot.set_iodo_queued(18, IoLevel::Low)
}

fn release<T>(dobot: &mut DobotClient<T>) -> Result<u64, failure::Error>
where
    T: Device,
{
    dobot.set_iodo_queued(17, IoLevel::Low)?;
    dobot.set_iodo_queued(18, IoLevel::High)
}

fn main() -> Result<(), failure::Error> {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap_or("/dev/tty.usbserial".to_owned());
    let d = SerialDevice::new(&path)?;
    let mut dobot = DobotClient::new(d);
    println!("SN number={:?}", dobot.get_device_sn()?);
    println!("{:?}", dobot.get_alarm_state()?);
    dobot.clear_all_alarm_state()?;
    //        std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("{:?}", dobot.get_alarm_state()?);
    let pose = dobot.get_pose()?;

    println!("{:?}", pose);

    /*
        let mut jog_params = dobot.get_jog_joint_params()?;
        jog_params.velocity[0] = 30.0;
        jog_params.velocity[1] = 30.0;
        jog_params.velocity[2] = 30.0;
        jog_params.velocity[3] = 60.0;
        println!("{:?}", jog_params);
        dobot.set_jog_joint_params(jog_params)?;
        dobot.set_jog_command(JogCommandType::Joint, JogCommand::CpDown)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        dobot.set_jog_command(JogCommandType::Joint, JogCommand::Idel)?;
    */
    /*
        dobot.set_jog_command(JogCommandType::Joint, JogCommand::BnDown)?;
        std::thread::sleep(std::time::Duration::from_millis(2000));
        dobot.set_jog_command(JogCommandType::Joint, JogCommand::Idel)?;
    */
    let common_param = PtpCommonParams {
        velocity_ratio: 30.0,
        acceleration_ratio: 10.0,
    };
    dobot.set_ptp_common_params(common_param)?;
    /*
        let ptp_joint_params = dobot.get_ptp_joint_params()?;
        println!("{:?}", ptp_joint_params);
        dobot.set_ptp_joint_params(ptp_joint_params)?;

        let ptp_coordinate_params = dobot.get_ptp_coordinate_params()?;
        println!("{:?}", ptp_coordinate_params);
        dobot.set_ptp_coordinate_params(ptp_coordinate_params)?;
    */
    //let ptp_jump_params = dobot.get_ptp_jump_params()?;
    //println!("{:?}", ptp_jump_params);
    /*
        let ptp_jump_params = PtpJumpParams {
            jump_height: 50.0,
            z_limit: 200.0,
            dummy: 0,
        };
        dobot.set_ptp_jump_params(ptp_jump_params)?;
    */
    println!("orientation {:?}", dobot.get_arm_orientation()?);
    dobot.set_arm_orientation(ArmOrientation::Righty)?;

    let cmd1 = PtpCommand {
        ptp_mode: PtpMode::JumpAngle,
        x: 80.0,
        y: 80.0,
        z: 100.0,
        r: 0.0,
    };
    dobot.set_ptp_command_queued(cmd1)?;

    let cmd1 = PtpCommand {
        ptp_mode: PtpMode::JumpXyz,
        x: 300.0,
        y: -130.0,
        z: 20.0,
        r: 0.0,
    };
    dobot.set_ptp_command_queued(cmd1)?;

    hold(&mut dobot)?;

    let cmd1 = PtpCommand {
        ptp_mode: PtpMode::JumpXyz,
        x: 0.0,
        y: 380.0,
        z: 40.0,
        r: 0.0,
    };
    dobot.set_ptp_command_queued(cmd1)?;
    release(&mut dobot)?;

    let cmd1 = PtpCommand {
        ptp_mode: PtpMode::MovjAngle,
        x: 80.0,
        y: 80.0,
        z: 100.0,
        r: 0.0,
    };
    dobot.set_ptp_command_queued(cmd1)?;

    /*
    println!("space = {}", dobot.get_queued_command_left_space()?);
    dobot.set_queued_command_clear()?;

    dobot.set_ptp_command(cmd1)?;
    //println!("cmd1: {}", index);
    println!("cur index = {}", dobot.get_queued_command_current_index()?);
    dobot.set_iodo_queued(17, IoLevel::Low)?;
    dobot.set_iodo_queued(18, IoLevel::Low)?;
    dobot.set_wait_command_queued(1000)?;
    // Vacuum Off
    dobot.set_iodo_queued(17, IoLevel::Low)?;
    dobot.set_iodo_queued(18, IoLevel::High)?;
    /*
        let cmd2 = PtpCommand {
            ptp_mode: PtpMode::MovjAngle,
            x: 80.0,
            y: 120.0,
            z: 100.0,
            r: 10.0,
        };
        let index2 = dobot.set_wait_command_queued(1000)?;
        println!("wait: {}", index2);
        let index3 = dobot.set_ptp_command_queued(cmd2)?;
        println!("cmd2: {}", index3);
    */
    dobot.set_queued_command_start_exec()?;

    println!("{:?}", dobot.get_alarm_state()?);

    for _ in 0..10 {
    std::thread::sleep(std::time::Duration::from_millis(200));
    println!("index = {}", dobot.get_queued_command_current_index()?);
    }
    dobot.set_queued_command_stop_exec()?;
     */
    let pose = dobot.get_pose()?;
    println!("{:?}", pose);
    println!("{:?}", dobot.get_alarm_state()?);
    dobot.clear_all_alarm_state()?;
    println!("{:?}", dobot.get_alarm_state()?);
    /*
    // Vacuum ON
    dobot.set_iodo(17, IoLevel::Low)?;
    dobot.set_iodo(18, IoLevel::Low)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    // Vacuum Off
    dobot.set_iodo(17, IoLevel::Low)?;
    dobot.set_iodo(18, IoLevel::High)?;
    */
    Ok(())
}
