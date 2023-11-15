use async_ringbuf::AsyncHeapRb;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let rb = AsyncHeapRb::<u8>::new(2);
    let (prod, cons) = rb.split();

    tokio::join!(
        async move {
            let mut n = prod;
            let s = format!("{}\t65000\t0\t{:?}\t1.1.1.1/24\t{{}}", Uuid::new_v4(), 1f64);
            for a in s.into_bytes() {
                println!("pushing {}", a);
                n.push(a);
            }
        },
        async move {
            let con = cons.compat();
            read_stuff(con).await;
        }
    );
}

async fn read_stuff(mut read: impl tokio::io::AsyncRead + Unpin) {
    let mut buf = [0u8];
    let res = read.read(&mut buf).await;

    if let Ok(1) = res {
        println!("Got {:?}", buf[0]);
    }
}
