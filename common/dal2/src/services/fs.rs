use std::io::SeekFrom;

use async_compat::CompatExt;
use async_trait::async_trait;
use common_exception::Result;
use tokio;
use tokio::io::AsyncSeekExt;

use crate::ops::Read;
use crate::ops::ReadBuilder;
use crate::ops::Reader;

/// TODO: https://github.com/datafuselabs/databend/issues/3677
#[derive(Default)]
pub struct Builder {}

impl Builder {
    pub fn finish(self) -> Backend {
        Backend {}
    }
}

/// TODO: https://github.com/datafuselabs/databend/issues/3677
pub struct Backend {}

impl Backend {
    pub fn build() -> Builder {
        Builder::default()
    }
}

#[async_trait]
impl<S: Send + Sync> Read<S> for Backend {
    async fn read(&self, args: &ReadBuilder<S>) -> Result<Reader> {
        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&args.path)
            .await
            .unwrap();

        if args.offset.is_some() {
            f.seek(SeekFrom::Start(args.offset.unwrap() as u64)).await?;
        }

        if args.size.is_some() {
            f.set_len(args.size.unwrap() as u64).await?;
        }

        Ok(Box::new(f.compat()))
    }
}
