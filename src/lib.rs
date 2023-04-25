use arducam_mipicamera::*;
use std::io::Cursor;
use image::io::Reader as ImageReader;

const V4L2_CID_FOCUS_ABSOLUTE: i32 = 10094858; // ((0x009a0000 | 0x900)+10)
const V4L2_CID_EXPOSURE: i32 = 9963793; 


/*
let cam = CameraInterface{
    i2c_bus: 0,       // /dev/i2c-0  or /dev/i2c-1 
    camera_num: camera_number,    // mipi interface num  
    sda_pins: [44, 0], // enable sda_pins[camera_num], disable sda_pins[camera_num ? 0 : 1] [28,0]
    scl_pins: [45, 1], // enable scl_pins[camera_num], disable scl_pins[camera_num ? 0 : 1] [29,1]
    led_pins:[30,2],
    shutdown_pins: [133,133],  //[31,3],
};
*/

/*
mode: 7, width: 1600, height: 600, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 8, width: 2560, height: 720, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 9, width: 3840, height: 1080, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 10, width: 5184, height: 1944, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 11, width: 6528, height: 1848, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
mode: 12, width: 6528, height: 2464, pixelformat: pRAA, desc: Used for Arducam synchronized stereo camera HAT
*/

pub fn capture(camera_number: i32) -> Result<(), Box<dyn std::error::Error>>{

    println!("Initializing Camera!");
    let mut camera = arducam_mipicamera::Camera::init(None).unwrap();
    //camera.set_lens_table();
    camera.set_mode(9);

    //println!("reseting control {} = {}", "V4L2_CID_FOCUS_ABSOLUTE", 10094858 );
    camera.reset_control(V4L2_CID_FOCUS_ABSOLUTE);  // todo:// this fails unwrap() ?
    camera.set_control( V4L2_CID_FOCUS_ABSOLUTE, 0 );
    camera.set_control( V4L2_CID_EXPOSURE, 1758 );
    
    let rslt = camera.set_resolution(3840,1080).unwrap();
    println!("resolution: {:?}", rslt);

    //println!("setting auto exposure");
    //camera.arducam_software_auto_exposure(true).unwrap();

    //println!("setting whitebalance");
    //camera.arducam_software_auto_white_balance(true).unwrap();

    println!("setting awb stuff");
    camera.arducam_manual_set_awb_compensation(100,100);

    let format = camera.get_format().unwrap();
    println!("\nformat: {:?}", format);

    // JPEG quality setting (1-100)
    println!("\nCapturing Image:");
    let buffer = camera.capture(5000, Encoding::Jpeg, 50).unwrap();

    let bytes = buffer.data();
    let img2 = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;


    img2.save("stereo_capture.jpg")?;

    Ok(())


}