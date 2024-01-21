// struct RandomizedSet {
//     root: Option<Box<Node>>
// }
//
// enum Node {
//     Empty,
//     Extension(Vec<u8>, Option<Box<Node>>),
//     Branch([Option<Box<Node>>;16], Option<Vec<u8>>),
//     Leaf(Vec<u8>),
// }
// // struct Node {
// //     val: u8,
// // }
//
//
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl RandomizedSet {
//
//     fn new() -> Self {
//         RandomizedSet {
//             root: None
//         }
//     }
//
//     fn insert(&self, val: i32) -> bool {
//         match  {  }
//     }
//
//     fn insert_at(&mut self, node: &mut Option<Box<Node>>, val: Vec<u8>, index: usize) -> Box<Node> {
//         match node {
//             None => Node::Leaf(val),
//             Some(mut old_root) => {
//                 match &mut old_root {
//                     Node::Leaf(old_val) => {
//                         let prefix = Self::prefix_len(&val[index..], &old_val[..]);
//                         match prefix {
//                             0 => {
//                                 let mut branch = [None;16];
//                                 branch[val[index]] = Some(Node::Leaf(val[index + 1..].to_vec()));
//                                 branch[old_val[0]] = Some(Node::Leaf(old_val[1..].to_vec()));
//                                 Box::new(Node::Branch(branch, None))
//                             },
//                             prefix if prefix == val.len() as u8 && prefix == old_val.len() as u8 => old_root,
//                             prefix => Box::new(Node::Extension(val[index..prefix].to_vec(), Some(Box::new(Node::Leaf(val[prefix..val.len()].to_vec())))))
//                         }
//                     },
//                     Node::Extension(old_val, next) => {
//                         let prefix = Self::prefix_len(&val[index..], &old_val[..]);
//                         match prefix {
//                             0 => {
//                                 let mut branch = [None;16];
//                                 branch[val[index]] = Some(Node::Leaf(val[index + 1..].to_vec()));
//
//                                 branch[old_val[0]] = self.insert_at(next, old_val[index + 1..].to_vec());
//                                 Box::new(Node::Branch(branch, None))
//                             },
//                             prefix if prefix == val.len() as u8 && prefix == old_val.len() as u8 => old_root,
//                             prefix => Box::new(Node::Extension(val[index..prefix].to_vec(), Some(Box::new(Node::Leaf(val[prefix..val.len()].to_vec())))))
//                         }
//                         if prefix == val.len() as u8 {
//                             old_root = self.insert_at(next, val, prefix as usize);
//                         } else {
//                             old_root =
//                         }
//                     }
//                 }
//             }
//         }
//         None
//     }
//
//     fn prefix_len(a: &[u8], b: &[u8]) -> u8 {
//         let a_len = a.len();
//         let b_len = b.len();
//         let mut i = 0;
//         while i < a_len && i < b_len && a[i] == a[b] {
//             i += 1;
//         }
//         i as u8
//     }
//
//     fn remove(&self, val: i32) -> bool {
//
//     }
//
//     fn get_random(&self) -> i32 {
//
//     }
// }