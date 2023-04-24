use arducam_mipicamera::*;
use std::io::Cursor;
use image::io::Reader as ImageReader;
/*
mode: 7, width: 1600, height: 600, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 8, width: 2560, height: 720, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 9, width: 3840, height: 1080, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 10, width: 5184, height: 1944, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 11, width: 6528, height: 1848, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 12, width: 6528, height: 2464, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {

    capture(1)?;
    Ok(())
}


pub fn capture(camera_number: i32) -> Result<(), Box<dyn std::error::Error>>{
    // let cam1 = CameraInterface{
    //     i2c_bus: 0,       // /dev/i2c-0  or /dev/i2c-1 
    //     camera_num: camera_number,    // mipi interface num
    //     sda_pins: [28,0], // enable sda_pins[camera_num], disable sda_pins[camera_num ? 0 : 1]
    //     scl_pins: [29,1], // enable scl_pins[camera_num], disable scl_pins[camera_num ? 0 : 1]
    //     led_pins:[30,2],
    //     shutdown_pins: [31,3],
    // };

    let cam = CameraInterface{
        i2c_bus: 0,       // /dev/i2c-0  or /dev/i2c-1 
        camera_num: camera_number,    // mipi interface num
        sda_pins: [44, 0], // enable sda_pins[camera_num], disable sda_pins[camera_num ? 0 : 1]
        scl_pins: [45, 1], // enable scl_pins[camera_num], disable scl_pins[camera_num ? 0 : 1]
        led_pins:[30,2],
        shutdown_pins: [133,133],
    };

    println!("Initializing Camera!");
    let mut camera = arducam_mipicamera::Camera::init(None).unwrap();
    //camera.set_lens_table();
    camera.set_mode(9);
    //resetGlobalParameter();

    //camera.set_control( V4L2_CID_FOCUS_ABSOLUTE, globalParam.focusVal )
    camera.set_control( 10094858, 0 );

    //camera.set_control( V4L2_CID_EXPOSURE, globalParam.exposureVal )
    camera.set_control( 9963793, 1758 );

    let rslt = camera.set_resolution(3840,1080).unwrap();
    println!("resolution: {:?}", rslt);

    println!("setting auto exposure");
    camera.arducam_software_auto_exposure(true).unwrap();

    println!("setting whitebalance");
    camera.arducam_software_auto_white_balance(true).unwrap();


    // let mode = camera.get_mode().unwrap();
    // let ctl = camera.get_control(2).unwrap();
    // println!("control: {:?}", ctl);

    let format = camera.get_format().unwrap();
    println!("\nformat: {:?}", format);

    // JPEG quality setting (1-100)
    println!("\nCapturing Image:");
    let buffer = camera.capture(5000, Encoding::Jpeg, 100).unwrap();

    let bytes = buffer.data();
    let img2 = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;


    img2.save("stereo_capture.png")?;

    Ok(())


}