use crate::backend::logging;
#[derive(Debug, Clone)]
pub struct ImageDataUri {
    pub mime: String,
    pub data_base64: String,
}

impl ImageDataUri {
    pub fn parse(uri: &str) -> Result<Self, String> {
        logging::debug("auto.image", "parse", "enter");
        let rest = uri
            .strip_prefix("data:")
            .ok_or_else(|| "image data uri must start with data:".to_string())?;
        let (mime, data) = rest
            .split_once(";base64,")
            .ok_or_else(|| "image data uri must contain ;base64,".to_string())?;
        if !mime.starts_with("image/") {
            return Err("data uri is not image mime".to_string());
        }
        Ok(Self {
            mime: mime.to_string(),
            data_base64: data.to_string(),
        })
    }

    pub fn to_uri(&self) -> String {
        logging::debug("auto.image", "to_uri", "enter");
        format!("data:{};base64,{}", self.mime, self.data_base64)
    }
}

#[derive(Debug, Clone)]
pub struct ImageContext {
    pub uri: String,
}

pub fn image_run(ctx: &ImageContext) -> Result<ImageDataUri, String> {
    logging::debug("auto.image", "image_run", "enter");
    ImageDataUri::parse(&ctx.uri)
}
