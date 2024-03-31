#[derive(Debug, Clone, Copy, Eq)]
struct Node {
    //describes how many time given character occurs, now same character can occurs for entire file
    freq: u128,

    //u8 represent of character
    character: u8,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.freq.cmp(&other.freq))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq.eq(&other.freq)
    }
}

// impl Eq for Node {}

fn read_and_count() {
    let mut file = match File::open("t1.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("error {e}");
            return;
        }
    };

    //todo: constraint remove this to be dynamic.
    let mut buffer: [u8; 1000] = [0; 1000];
    let size = match file.read(&mut buffer) {
        Ok(s) => s,
        Err(e) => {
            println!("failed to read file {}", e);
            return;
        }
    };

    let buffer_str = String::from_utf8(buffer.to_vec()).expect("unhandled string  conversion");
    println!("fetched a file size {size} content\n {buffer_str}",);

    let mut char_freq: [Node; 256] = [Node {
        freq: 0,
        character: 0,
    }; 256];

    for i in 0..size {
        // if char_freq[buffer[i] as usize].freq == u128::MAX {
        //     char_freq[buffer[i] as usize].freq = 0
        // }

        char_freq[buffer[i] as usize].freq += 1;
        char_freq[buffer[i] as usize].character = buffer[i];
    }

    let mut char_freq_vec: Vec<Node> = Vec::new();
    let mut unique_chars: usize = 0;
    char_freq.into_iter().for_each(|f| {
        if f.character > 0 {
            println!("{:?} : {:?}", f.character, f.freq);
            char_freq_vec.push(f);
            unique_chars += 1
        }
    });

    for i in 0..unique_chars - 1 {
        heapify(&mut char_freq_vec, 1, unique_chars - i, i, HeapType::min);
        let val1 = char_freq_vec[i];
        let val2 = char_freq_vec[i + 1];
        char_freq_vec[i + 1].freq = val1.freq + val2.freq
    }
    println!("{:#?}", char_freq_vec[unique_chars - 1]);
    println!("{:#?}", char_freq)
}
enum HeapType {
    min,
    max,
}

fn heapify<T: Ord + Copy>(
    v: &mut [T],
    start: usize,
    end: usize,
    offset: usize,
    heap_type: HeapType,
) {
    match heap_type {
        HeapType::min => {
            let mid: usize = end / 2 + offset;

            for i in (start..=mid).rev() {
                let idx = i - 1 + offset;
                let right: usize = 2 * i - 1 + offset;
                let left: usize = 2 * i + offset;

                // println!("idx {idx}");
                // println!("right {right}");
                // println!("left {left}");

                if left < (end + offset) && v[idx] > v[left] {
                    let temp = v[idx];
                    v[idx] = v[left];
                    v[left] = temp;
                }

                if right < (end + offset) && v[idx] > v[right] {
                    let temp = v[idx];
                    v[idx] = v[right];
                    v[right] = temp;
                }
            }
        }
        _ => println!("unhandled type"),
    }
}

//assumption heap is min heap
