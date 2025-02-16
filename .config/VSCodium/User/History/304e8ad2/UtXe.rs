use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::imgcodecs::imwrite;
use opencv::core::Mat;
use opencv::highgui::{imshow, wait_key};
use opencv::objdetect::{CascadeClassifier};
use opencv::core::Vector;

fn main() -> opencv::Result<()> {
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    let mut frame = Mat::default();

    let mut rpalm = CascadeClassifier::new("./src/rpalm.xml")?;
    let mut lpalm = CascadeClassifier::new("./src/lpalm.xml")?;
    let mut fist = CascadeClassifier::new("./src/fist.xml")?;

    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = Mat::default();
            opencv::imgproc::cvt_color(&frame, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0)?;
            
            let mut fists = Vector::new();
            let mut rpalms = Vector::new();
            let mut lpalms = Vector::new();
            rpalm.detect_multi_scale(&gray, &mut rpalms, 1.1, 3, 0, opencv::core::Size::default(), opencv::core::Size::default())?;

            lpalm.detect_multi_scale(&gray, &mut lpalms, 1.1, 3, 0, opencv::core::Size::default(), opencv::core::Size::default())?;
            fist.detect_multi_scale(&gray, &mut fists, 1.1, 3, 0, opencv::core::Size::default(), opencv::core::Size::default())?;
            for obj in fists {
                opencv::imgproc::rectangle(&mut frame, obj, opencv::core::Scalar::new(0.0,0.0,255.0,0.0), 2, 8, 0)?;
            }
            for obj in rpalms {
                opencv::imgproc::rectangle(&mut frame, obj, opencv::core::Scalar::new(0.0,255.0,0.0,0.0), 2, 8, 0)?;
            }
            for obj in lpalms {
                opencv::imgproc::rectangle(&mut frame, obj, opencv::core::Scalar::new(255.0,0.0,0.0,0.0), 2, 8, 0)?;
            }

            imshow("Detection", &frame)?;
            let key = wait_key(30)?;
            if key == 27 {
                break;
            }
        }
    }

    Ok(())
}

