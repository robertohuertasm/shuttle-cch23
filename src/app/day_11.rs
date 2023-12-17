use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::Responder;
use image::GenericImageView;

pub async fn decoration() -> impl Responder {
    NamedFile::open_async("assets/decoration.png").await
}

#[derive(Debug, MultipartForm)]
pub struct Form11 {
    pub image: Vec<actix_multipart::form::bytes::Bytes>,
}

pub async fn red_pixels(form: MultipartForm<Form11>) -> impl Responder {
    // red > blue + green
    let image = form.into_inner().image;

    if let Some(img) = image.first() {
        red_pixels_impl(&img.data).await.to_string()
    } else {
        "0".to_owned()
    }
}

async fn red_pixels_impl(data: &[u8]) -> u32 {
    if let Ok(image) = image::load_from_memory(data) {
        let mut red_pixels = 0;
        for pixel in image.pixels() {
            let rgba = pixel.2;
            let (red, green, blue) = (rgba[0], rgba[1], rgba[2]);
            let green_blue = green.checked_add(blue);
            if let Some(gb) = green_blue {
                if red > gb {
                    red_pixels += 1;
                }
            }
        }
        red_pixels
    } else {
        0
    }
}
