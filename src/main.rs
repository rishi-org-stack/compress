use std::{fs::File, io::Read};

fn main() {
    let mut file = match File::open("java.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("error {e}");
            return;
        }
    };

    let mut buffer: [u8; 124] = [0; 124];
    let size = match file.read(&mut buffer) {
        Ok(s) => s,
        Err(e) => {
            println!("failed to read file {}", e);
            return;
        }
    };

    let mut char_freq: [u32; 256] = [0; 256];
    buffer.into_iter().for_each(|v| char_freq[v as usize] += 1);
    let buffer_str = String::from_utf8(buffer.to_vec()).expect("unhandled string  conversion");
    println!("fetched a file size {size} content\n {buffer_str}",);
    println!("freq {:#?}", char_freq);
}
// TODO
// 1. reading a file
// 2. calculate frequency
// 3. create min head on frequency
// 4. iterate unless only 1 node is present fetch 2 min nodes make a root out of that push in heap
// 5. traverse through the tree if left its 0 bit it right its 1 bit
// 6. write to file combination of 0's and one
// 7. make header if 1st bit is 1 then next next 8bits represent value, append this to file head.
// 8. reconstruct  tree from header if starts from 1bit make a node and push if 0 pop 2 and make a node push in tree
// 9. traverse through bit encoded value to reconstruct the string
https://engineering.purdue.edu/ece264/21sp/hw/HW15?alt=huffman