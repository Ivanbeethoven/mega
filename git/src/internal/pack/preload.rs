use std::{collections::HashMap, io::Read};
use crate::{internal::{pack::Hash, zlib::stream::inflate::ReadPlain}, utils, errors::GitError};
use tokio::sync::RwLock;
use super::Pack;
///
/// 
/// 
/// 
/// 

struct Entry{
    header: EntryHeader,
    data:Vec<u8>,
}
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

            println!("offset :{},type :{}",offset,type_num);
            entries.push(Entry { header, data: content });
            map.insert(offset, i);
            offset += iter_offset;
        }
        PackPreload { map, entries}
    }
}

pub fn decod_load(p: PackPreload){
    let share = RwLock::new(p);
}

#[cfg(test)]
mod tests{
    use std::{fs::File, path::Path, io:: BufReader};



    use crate::internal::pack::preload::PackPreload;

    #[test]
    fn preload_read_decode(){

        let mut file = File::open(Path::new(
            "../tests/data/packs/pack-d50df695086eea6253a237cb5ac44af1629e7ced.pack",
        ))
        .unwrap();

        let p = PackPreload::new(BufReader::new(file));

    }

}