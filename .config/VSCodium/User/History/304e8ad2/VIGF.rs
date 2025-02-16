use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::core::Scalar;
use opencv::imgproc::LINE_8;
use opencv::imgcodecs::imwrite;
use opencv::imgproc::{cvt_color, Canny, find_contours, draw_contours, COLOR_BGR2GRAY, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE};
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

    // loop {
    //     cam.read(&mut frame)?;
    //     if frame.size()?.width > 0 {
    //         let mut gray = Mat::default();
    //         opencv::imgproc::cvt_color(&frame, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0)?;
            
    //         let mut fists = Vector::new();
    //         let mut rpalms = Vector::new();
    //         let mut lpalms = Vector::new();
    //         rpalm.detect_multi_scale(&gray, &mut rpalms, 1.1, 3, 0, opencv::core::Size::default(), opencv::core::Size::default())?;

    //         lpalm.detect_multi_scale(&gray, &mut lpalms, 1.1, 3, 0, opencv::core::Size::default(), opencv::core::Size::default())?;
    //         fist.detect_multi_scale(&gray, &mut fists, 1.1, 3, 0, opencv::core::Size::default(), opencv::core::Size::default())?;
    //         for obj in fists {
    //             opencv::imgproc::rectangle(&mut frame, obj, opencv::core::Scalar::new(0.0,0.0,255.0,0.0), 2, 8, 0)?;
    //         }
    //         for obj in rpalms {
    //             opencv::imgproc::rectangle(&mut frame, obj, opencv::core::Scalar::new(0.0,255.0,0.0,0.0), 2, 8, 0)?;
    //         }
    //         for obj in lpalms {
    //             opencv::imgproc::rectangle(&mut frame, obj, opencv::core::Scalar::new(255.0,0.0,0.0,0.0), 2, 8, 0)?;
    //         }

    //         imshow("Detection", &frame)?;
    //         let key = wait_key(30)?;
    //         if key == 27 {
    //             break;
    //         }
    //     }
    // }
    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            // Convert frame to grayscale
            let mut gray = Mat::default();
            cvt_color(&frame, &mut gray, COLOR_BGR2GRAY, 0)?;

            // Apply Canny edge detection
            let mut edges = Mat::default();
            Canny(&gray, &mut edges, 100.0, 200.0, 3)?;

            // Find contours
            let mut contours = Vector::new();
            find_contours(&edges, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE, opencv::core::Point::new(0, 0))?;

            // Draw contours
            let contour_color = Scalar::new(0.0, 255.0, 0.0, 0.0); // Green color in BGR
            let mut contour_frame = frame.clone();
            draw_contours(&mut contour_frame, &contours, -1, contour_color, 2, opencv::core::LINE_8, &Mat::default(), 0, opencv::core::Point::new(0, 0))?;

            // Display the result
            imshow("Edges", &contour_frame)?;
            let key = wait_key(30)?; // Wait for 30 ms
            if key == 27 { // ESC key to exit
                break;
            }
        }
    }


    Ok(())
}

