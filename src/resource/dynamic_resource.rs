use std::str::FromStr;

use super::{disk_resource::DiskResource, image_resource::ImageResource, remote_resource::RemoteResource, ResourceError};


pub enum DynamicResource {
    DiskResource(DiskResource),
    ImageResource(ImageResource),
    RemoteResource(RemoteResource)
}


impl FromStr for DynamicResource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let resource = ImageResource::from_str(s);

        if resource.is_ok() {
            return Ok(Self::ImageResource(resource.unwrap()))
        }

        let resource = DiskResource::from_str(s);

        if resource.is_ok() {
            return Ok(Self::DiskResource(resource.unwrap()))
        }

        let resource = RemoteResource::from_str(s);

        if resource.is_ok() {
            return Ok(Self::RemoteResource(resource.unwrap()))
        }

        Err(ResourceError::InvalidResource)
    }
}