#[derive(Debug)]
struct Node {
    val: u8,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

impl Node {
    fn new(left_node: Node, right_node: Node) -> Node {
        Node {
            val: right_node.val + left_node.val,
            right: Some(Box::new(right_node)),
            left: Some(Box::new(left_node)),
        }
    }

    fn iter(&self) {
        if let Some(right) = &self.right {
            println!("r {}", self.val);
            let mut right_iter = right;

            loop {
                println!("r {}", right_iter.val);
                if let Some(r) = &right_iter.right {
                    right_iter = r;
                } else {
                    break;
                }
            }
        }
    }

    // fn add_left(&self, left_node: Node) {
    //     let right = Some(Box::new(*self));
    //     let left = Some(Box::new(left_node));
    //     Node {
    //         val: self.val + left_node.val,
    //         right: right,
    //         left: Some(Box::new(left_node)),
    //     };
    // }
}

fn main() {
    let left = Node {
        val: 1,
        left: None,
        right: None,
    };

    let right = Node {
        val: 1,
        left: None,
        right: None,
    };

    let mut head = Node::new(left, right);

    head = Node::new(
        Node {
            val: 2,
            left: None,
            right: None,
        },
        head,
    );
    head = Node::new(
        Node {
            val: 4,
            left: None,
            right: None,
        },
        head,
    );
    head.iter();
    println!("{:#?}", head)
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
// https://engineering.purdue.edu/ece264/21sp/hw/HW15?alt=huffman
// https://rust-unofficial.github.io/too-many-lists/first-layout.html
