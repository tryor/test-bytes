use std::sync::Arc;
use bytestring::ByteString;
use bytes::Bytes;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

#[derive(Clone)]
struct ID(ByteString);

impl ID {
    fn len(&self) -> usize {
        1
    }
}

impl Drop for ID {
    fn drop(&mut self) {
        // println!("{:?}, {:?}", self.0, std::thread::current().id())
        //drop(&self.0)
    }
}

fn main() {
    {
        let mut datas = Vec::new();
        let data = Bytes::copy_from_slice(&[0, 1, 2, 3]);
        let id = ID(ByteString::try_from(data).unwrap());
        for _ in 0..10000000 {
            datas.push(id.clone())
        }

        println!("{:?}", datas.len())
    }
    std::thread::sleep(Duration::from_secs(1));

}

//
// use tokio::io::{AsyncRead, AsyncWrite};
// use tokio::net::{TcpListener, TcpStream};
// use tokio::stream::Stream;
//
// fn main(){
//     let mut rt = tokio::runtime::Builder::new()
//         .threaded_scheduler()
//         .core_threads(8)
//         .max_threads(256)
//         .thread_name("worker")
//         .thread_stack_size(4 * 1024 * 1024)
//         .enable_all()
//         .build().unwrap();
//
//     let runner = async {
//         let count = Arc::new(AtomicUsize::new(0));
//
//         let count_d = count.clone();
//         tokio::spawn(async move {
//             loop {
//                 std::thread::sleep(Duration::from_secs(3));
//                 println!("count: {}", count_d.load(Ordering::SeqCst));
//             }
//         });
//
//         loop {
//             let run = |data: bytes::Bytes| async {
//                 // let data = Bytes::copy_from_slice(&[0, 1, 2, 3]);
//                 // let id = Arc::new(ID(ByteString::try_from(data).unwrap()));
//                 let id = ID(ByteString::try_from(data).unwrap());
//                 // let id = ID(ByteString::from(String::from("1323432432")));
//                 let id1 = id.clone();
//                 let id2 = id.clone();
//                 let id3 = id.clone();
//                 let id4 = id.clone();
//                 let count1 = count.clone();
//                 let count2 = count.clone();
//                 let count3 = count.clone();
//                 let count4 = count.clone();
//                 let t1 = tokio::spawn(async move {
//                     //println!("id1: {}", id1);
//                     count1.fetch_add(id1.len(), Ordering::SeqCst);
//                 });
//
//
//                 let t2 = tokio::spawn(async move {
//                     // println!("id2: {}", id2);
//                     count2.fetch_add(id2.len(), Ordering::SeqCst);
//                 });
//
//                 let t3 = tokio::spawn(async move {
//                     //println!("id3: {}", id3);
//                     count3.fetch_add(id3.len(), Ordering::SeqCst);
//                 });
//
//                 let t4 = tokio::spawn(async move {
//                     //println!("id4: {}", id4);
//                     count4.fetch_add(id4.len(), Ordering::SeqCst);
//                 });
//
//                 t1.await.unwrap();
//                 t2.await.unwrap();
//                 t3.await.unwrap();
//                 t4.await.unwrap();
//             };
//
//             let mut runs = Vec::new();
//
//             runs.push(run(Bytes::copy_from_slice(&[0, 1, 2, 3])));
//             runs.push(run(Bytes::from("test...".to_string())));
//             runs.push(run(Bytes::from(vec![22,33,22,34,34])));
//             runs.push(run(Bytes::from("aaaaaaaaaa")));
//             runs.push(run(Bytes::from_static(&[23,3,23])));
//
//             runs.push(run(Bytes::from_static(&[23,3,23])));
//
//
//
//             futures::future::join_all(runs).await;
//
//         }
//
//
//     };
//
//     rt.block_on(runner);
// }



// fn main2() {
//     let count = Arc::new(AtomicUsize::new(0));
//
//     let count_d = count.clone();
//     std::thread::spawn(move || {
//         loop {
//             std::thread::sleep(Duration::from_secs(3));
//             println!("count: {}", count_d.load(Ordering::SeqCst));
//         }
//     });
//
//     loop {
//         let data = Bytes::copy_from_slice(&[0, 1, 2, 3]);
//         let id = Arc::new(ID(ByteString::try_from(data).unwrap()));
//         // let id = ID(ByteString::try_from(data).unwrap());
//         // let id = ID(ByteString::from(String::from("1")));
//         let id1 = id.clone();
//         let id2 = id.clone();
//         let id3 = id.clone();
//         let id4 = id.clone();
//         let count1 = count.clone();
//         let count2 = count.clone();
//         let count3 = count.clone();
//         let count4 = count.clone();
//         let t1 = std::thread::spawn(move || {
//             //println!("id1: {}", id1);
//             count1.fetch_add(id1.len(), Ordering::SeqCst);
//         });
//
//
//         let t2 = std::thread::spawn(move || {
//             // println!("id2: {}", id2);
//             count2.fetch_add(id2.len(), Ordering::SeqCst);
//         });
//
//         let t3 = std::thread::spawn(move || {
//             //println!("id3: {}", id3);
//             count3.fetch_add(id3.len(), Ordering::SeqCst);
//         });
//
//         let t4 = std::thread::spawn(move || {
//             //println!("id4: {}", id4);
//             count4.fetch_add(id4.len(), Ordering::SeqCst);
//         });
//
//         t1.join().unwrap();
//         t2.join().unwrap();
//         t3.join().unwrap();
//         t4.join().unwrap();
//     }
// }
