use std::convert::AsRef;
use std::path::Path;
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat};
use either::{Either, Either::*};

use crate::data::{Resolution, OutputFormat};
use crate::codec::jpeg;
use crate::codec::png;
use crate::codec::webp;

pub struct OptJob {
    source: DynamicImage,
    source_format: ImageFormat,
    output_format: OutputFormat,
    max_size: Option<Resolution>,
}

impl OptJob {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let source = std::fs::read(path).expect("input file path");
        OptJob::new(&source)
    }
    pub fn new(source: &[u8]) -> Result<Self, ()> {
        let source_format = ::image::guess_format(source).map_err(drop)?;
        let output_format = match source_format {
            ImageFormat::JPEG => OutputFormat::Jpeg,
            ImageFormat::PNG => OutputFormat::Png,
            ImageFormat::WEBP => OutputFormat::Webp,
            _ => OutputFormat::Jpeg
        };
        match source_format {
            ImageFormat::WEBP => {
                let source = webp::decode::decode(source);
                let source = crate::data::ensure_even_reslution(&source);
                Ok(OptJob {
                    output_format,
                    source,
                    source_format,
                    max_size: None,
                })
            }
            _ => {
                let source = ::image::load_from_memory_with_format(
                        source,
                        source_format,
                    )
                    .map_err(drop)?;
                let source = crate::data::ensure_even_reslution(&source);
                Ok(OptJob {
                    output_format,
                    source,
                    source_format,
                    max_size: None,
                })
            }
        }
    }
    pub fn output_format(&mut self, output_format: OutputFormat) {
        self.output_format = output_format;
    }
    pub fn max_size(&mut self, max_size: Resolution) {
        self.max_size = Some(max_size);
    }
    pub fn run(self) -> Result<Vec<u8>, ()> {
        let input = match self.max_size {
            Some(res) if (res.width, res.height) > self.source.dimensions() => {
                self.source.resize(res.width, res.height, ::image::FilterType::Lanczos3)
            },
            _ => self.source.clone(),
        };
        match self.output_format {
            OutputFormat::Webp => {
                Ok(webp::opt::opt(&input).0)
            }
            OutputFormat::Jpeg => {
                Ok(jpeg::OptContext::from_image(input.clone()).run_search().0)
            }
            OutputFormat::Png => {
                Ok(png::basic_optimize(&input))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opt_basic() {
        let test_image = include_bytes!("../assets/test/1.jpeg");
        for output_format in vec![OutputFormat::Jpeg, OutputFormat::Png, OutputFormat::Webp] {
            let mut opt_job = OptJob::new(test_image).expect("new opt job");
            opt_job.output_format(output_format);
            opt_job.max_size(Resolution::new(1000, 1000));
            let result = opt_job.run();
            assert!(result.is_ok());
        }
    }
}