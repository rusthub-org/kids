use futures::AsyncWriteExt;
use std::pin::Pin;
use async_std::{
    io::{self, Read},
    stream::Stream,
    fs::File,
    path::PathBuf,
    task::{Context, Poll},
};
use tide::Request;
use multer::Multipart;

use crate::State;

// Process the request body as multipart/form-data.
pub async fn file_copy(
    req: Request<State>,
    file_path: PathBuf,
) -> Result<(), io::Error> {
    let boundary =
        req.content_type().unwrap().param("boundary").unwrap().to_string();
    let body_stream = BufferedBytesStream { inner: req };

    let mut multipart = Multipart::new(body_stream, boundary);
    let multipart_field = multipart.next_field().await.unwrap();
    let field_chunk = multipart_field.unwrap().chunk().await.unwrap();

    let mut file = File::create(file_path).await?;
    let file_copy = file.write_all(&field_chunk.unwrap()).await;

    if file_copy.is_ok() {
        file.flush().await
    } else {
        file_copy
    }
}

#[derive(Debug)]
struct BufferedBytesStream<T> {
    inner: T,
}

impl<T: Read + Unpin> Stream for BufferedBytesStream<T> {
    type Item = async_std::io::Result<Vec<u8>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut buf = [0u8; 2048];

        let rd = Pin::new(&mut self.inner);
        match futures::ready!(rd.poll_read(cx, &mut buf)) {
            Ok(0) => Poll::Ready(None),
            Ok(n) => Poll::Ready(Some(Ok(buf[..n].to_vec()))),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}
