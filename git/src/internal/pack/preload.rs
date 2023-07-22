use std::{collections::HashMap, io::Read, sync::{mpsc, Arc}, thread};
use crate::{internal::{pack::Hash, zlib::stream::inflate::ReadPlain}, utils, errors::GitError};
use tokio::{sync::RwLock, runtime::Runtime};
use super::Pack;
use num_cpus;
///
/// 
/// 
/// 
/// 

struct Entry{
    header: EntryHeader,
    offset: usize,
    data:Vec<u8>,
}
#[derive(Debug)]
pub enum EntryHeader {
    Commit,
    Tree,
    Blob,
    Tag,
    RefDelta { base_id: Hash },
    OfsDelta { base_distance: usize  },
}
pub struct PackPreload{
    map: HashMap<usize,usize>,//Offset -> iterator in entity
    entries: Vec<Entry>,
}

impl PackPreload {
    pub fn new<R>(mut r : R) -> PackPreload  where R:std::io::BufRead{
        let mut offset: usize  = 12;

        let pack = Pack::check_header(&mut r).unwrap();
        let mut map = HashMap::new();
        let obj_number = pack.number_of_objects();
        let mut entries = Vec::with_capacity(obj_number);
        for i in 0..obj_number{
            
            let mut iter_offset: usize = 0;
            // Read the Object Type and Total Size of one Object
            let (type_num, size) = utils::read_type_and_size(&mut r).unwrap();
            //Get the Object according to the Types Enum
            iter_offset += utils::get_7bit_count(size << 3);
            let header: EntryHeader;
            match type_num{
                1 => header = EntryHeader::Commit,
                2 => header = EntryHeader::Tree,
                3 => header = EntryHeader::Blob,
                4 => header = EntryHeader::Tag,

                6 => {
                    let delta_offset = utils::read_offset_encoding(&mut r, &mut iter_offset)
                    .unwrap() as usize;
                   
                    // Count the base object offset and get the base object from the cache in EntriesIter
                    let base_offset = offset
                        .checked_sub(delta_offset)
                        .ok_or_else(|| {
                            GitError::InvalidObjectInfo("Invalid OffsetDelta offset".to_string())
                        })
                        .unwrap();
                    header = EntryHeader::OfsDelta{base_distance:base_offset};
                },
                7 => {
                    // Ref Delta Object
                    let hash = utils::read_hash(&mut r).unwrap();
                    iter_offset += 20;
                    header = EntryHeader::RefDelta{base_id:hash};
                }, 
                _ => todo!(),//error
            }
            let mut reader=ReadPlain::new(&mut r);
            let mut content =Vec::new();
            reader.read_to_end(&mut content).unwrap();
            iter_offset+= reader.decompressor.total_in() as usize;

            //println!("offset :{},type :{}",offset,type_num);
            entries.push(Entry { header,offset, data: content });
            map.insert(offset, i);
            offset += iter_offset;
        }
        PackPreload { map, entries}
    }
}

pub async fn decod_load(p: PackPreload) -> (){
    let a_cpu =  num_cpus::get();
    let share =Arc::new(RwLock::new(p)) ;
    // 创建一个多生产者，单消费者的通道
    let (tx, rx) = mpsc::channel();
    println!("begin_____");

    // 启动三个生产者任务
    let producer_handles: Vec<_> = (0..3)
        .map(|i| {
            let tx_clone = tx.clone();
            thread::spawn(
               move || produce_data(i, tx_clone)
            )
        })
        .collect();
    // 使用 tokio::runtime::Runtime 创建异步运行时
    let rt: Runtime = Runtime::new().unwrap();
    // 启动消费者任务
    let consume_handle = thread::spawn( move ||{
        rt.block_on(consume_data(rx))
    });

    // 等待所有生产者任务结束
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // 关闭通道以结束消费者任务
    drop(tx);

    // 等待消费者任务结束
    consume_handle.join().unwrap();

    ()
}


fn produce_data(id: usize, tx: mpsc::Sender<usize>) {
    for i in 0..200 {
        // 模拟一些计算密集型任务
        let value = i + id * 1000;
        for _ in 0..10{}
        tx.send(value).unwrap();
    }
}

async fn consume_data(rx: mpsc::Receiver<usize>) -> (){
    for data in rx {
        // 这里可以处理消费收到的数据，例如打印出来
        println!("Received data: {}", data);
    }
    println!("Consumer task finished.");
    ()
}

#[cfg(test)]
mod tests{
    use std::{fs::File, path::Path, io:: BufReader};



    use crate::internal::pack::preload::PackPreload;

    use super::decod_load;
    use tokio::test;

    #[test]
    async fn preload_read_decode(){

        let file = File::open(Path::new(
            "../tests/data/packs/pack-d50df695086eea6253a237cb5ac44af1629e7ced.pack",
        ))
        .unwrap();

        let p = PackPreload::new(BufReader::new(file));
        for it in p.entries{
            println!("{:?},offset:{}",it.header,it.offset);
        }
    }

    #[test]
    async fn test_demo_channel(){
        let file = File::open(Path::new(
            "../tests/data/packs/pack-d50df695086eea6253a237cb5ac44af1629e7ced.pack",
        ))
        .unwrap();

        let p = PackPreload::new(BufReader::new(file));
        decod_load(p).await;
    }

}