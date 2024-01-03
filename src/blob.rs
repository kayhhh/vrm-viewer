use bevy::{
    asset::io::{AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader},
    prelude::*,
    utils::BoxedFuture,
};
use std::pin::pin;
use std::{future::Future, path::Path};

#[derive(Default)]
struct BlobReader;

impl AssetReader for BlobReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        // self.0.read(path)
    }
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        pin!(Box::new())
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        // self.0.read_directory(path)
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        // self.0.is_directory(path)
    }
}

struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Name("blob".into()),
            AssetSource::build().with_reader(|| Box::new(BlobReader)),
        );
    }
}
