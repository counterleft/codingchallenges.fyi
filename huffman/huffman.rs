use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/*
* The thing that doesn't work in writing/reading the binary for the huffman compressed text
*/

fn calculate_frequency(text: &str) -> HashMap<char, u32> {
    let mut result = HashMap::new();

    for c in text.chars() {
        let count = match result.get(&c) {
            Some(&count) => count,
            None => 0,
        };

        result.insert(c, count + 1);
    }

    result
}

fn get_file_contents(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => (),
    }

    s
}

#[derive(Debug, PartialEq)]
struct HuffmanNode {
    weight: u32,
    value: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

fn build_huffman_tree(freqs: &HashMap<char, u32>) -> Option<Box<HuffmanNode>> {
    let mut queue: Vec<Box<HuffmanNode>> = vec![];

    for (character, frequency) in freqs.iter() {
        queue.push(Box::new(HuffmanNode {
            weight: *frequency,
            value: Some(*character),
            left: None,
            right: None,
        }))
    }

    // Sorted highest-to-lowest because vec.pop() removes the last elem and
    // we want the lowest weight elem
    // queue.sort_by(|a, b| b.weight.cmp(&a.weight));
    sort_queue(&mut queue);

    // println!("QUEUE: {:?}", queue);

    while queue.len() > 1 {
        let merged = match (queue.pop(), queue.pop()) {
            (Some(first_boxed_node), Some(second_boxed_node)) => Box::new(HuffmanNode {
                weight: first_boxed_node.weight + second_boxed_node.weight,
                value: None,
                left: Some(first_boxed_node),
                right: Some(second_boxed_node),
            }),
            (_, _) => panic!("Not possible."),
        };

        queue.push(merged);
        sort_queue(&mut queue);
    }

    queue.pop()
}

fn sort_queue(queue: &mut Vec<Box<HuffmanNode>>) {
    queue.sort_by(|a, b| {
        if b.weight.cmp(&a.weight) == Ordering::Equal {
            b.value.cmp(&a.value)
        } else {
            b.weight.cmp(&a.weight)
        }
    });
}

#[derive(Debug, PartialEq)]
struct HuffmanCode {
    value: char,
    freq: u32,
    code: String,
}

impl HuffmanCode {
    fn bits(&self) -> usize {
        self.code.len()
    }
}

fn generate_huffman_codes(tree: &HuffmanNode) -> Vec<HuffmanCode> {
    // traverse the tree in a depth-first manner and calculate Codes along the way
    let mut result: Vec<HuffmanCode> = vec![];

    let mut code = String::new();
    traverse_tree(&tree, &mut result, &mut code);

    result
}

fn traverse_tree(tree: &HuffmanNode, result: &mut Vec<HuffmanCode>, code: &mut String) {
    match tree.value {
        Some(v) => {
            result.push(HuffmanCode {
                value: v,
                freq: tree.weight,
                code: code.clone(),
            });
        }
        None => {
            if let Some(left) = &tree.left {
                code.push('0');
                traverse_tree(&left, result, code);
                code.pop();
            }
            if let Some(right) = &tree.right {
                code.push('1');
                traverse_tree(&right, result, code);
                code.pop();
            }
        }
    }
}

fn main() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_frequency() {
        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 1);
        expected.insert('c', 2);
        expected.insert('d', 1);

        assert_eq!(calculate_frequency("abccd"), expected);
    }

    #[test]
    fn test_calculate_frequency_special_chars() {
        let mut expected = HashMap::new();
        expected.insert('b', 2);
        expected.insert('o', 4);
        expected.insert('\'', 1);
        expected.insert('$', 1);
        expected.insert(' ', 2);
        expected.insert('c', 1);
        expected.insert('k', 1);
        expected.insert('i', 1);
        expected.insert('e', 1);
        expected.insert('s', 2);
        expected.insert('!', 1);
        expected.insert('w', 2);
        expected.insert('_', 1);
        expected.insert(',', 1);

        assert_eq!(calculate_frequency("bob's $cookies!, wow_"), expected);
    }

    #[test]
    fn test_calculate_frequency_les_mis() {
        let text = get_file_contents("lesmis.txt");

        let expected: Option<&u32> = Some(&333);
        assert_eq!(calculate_frequency(&text).get(&'X'), expected);

        let expected: Option<&u32> = Some(&223000);
        assert_eq!(calculate_frequency(&text).get(&'t'), expected);
    }

    #[test]
    fn test_build_huffman_tree() {
        let mut freqs = HashMap::new();
        freqs.insert('c', 32);
        freqs.insert('d', 42);

        let expected = Box::new(HuffmanNode {
            weight: 74,
            value: None,
            left: Some(Box::new(HuffmanNode {
                weight: 32,
                value: Some('c'),
                left: None,
                right: None,
            })),
            right: Some(Box::new(HuffmanNode {
                weight: 42,
                value: Some('d'),
                left: None,
                right: None,
            })),
        });

        assert_eq!(build_huffman_tree(&freqs), Some(expected));
    }

    #[test]
    fn test_build_huffman_tree_empty_input() {
        assert_eq!(build_huffman_tree(&HashMap::new()), None);
    }

    #[test]
    fn test_build_huffman_tree_single_entry_input() {
        let mut freqs = HashMap::new();
        freqs.insert('e', 331);

        let expected = Box::new(HuffmanNode {
            weight: 331,
            value: Some('e'),
            left: None,
            right: None,
        });

        assert_eq!(build_huffman_tree(&freqs), Some(expected));
    }

    #[test]
    fn test_build_huffman_tree_three_entry_input() {
        let mut freqs = HashMap::new();
        freqs.insert('c', 1);
        freqs.insert('d', 22);
        freqs.insert('e', 33);

        let expected = Box::new(HuffmanNode {
            weight: 56,
            value: None,
            left: Some(Box::new(HuffmanNode {
                weight: 23,
                value: None,
                left: Some(Box::new(HuffmanNode {
                    weight: 1,
                    value: Some('c'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(HuffmanNode {
                    weight: 22,
                    value: Some('d'),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(HuffmanNode {
                weight: 33,
                value: Some('e'),
                left: None,
                right: None,
            })),
        });

        // println!("{:?}", expected);
        assert_eq!(build_huffman_tree(&freqs), Some(expected));
    }

    #[test]
    fn test_build_huffman_tree_webexample() {
        // This list is from the example tree-build at
        // https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html
        let mut freqs = HashMap::new();

        freqs.insert('c', 32);
        freqs.insert('d', 42);
        freqs.insert('e', 120);
        freqs.insert('k', 7);
        freqs.insert('l', 42);
        freqs.insert('m', 24);
        freqs.insert('u', 37);
        freqs.insert('z', 2);

        let expected = Box::new(HuffmanNode {
            weight: 306,
            value: None,
            left: Some(Box::new(HuffmanNode {
                weight: 120,
                value: Some('e'),
                left: None,
                right: None,
            })),
            right: Some(Box::new(HuffmanNode {
                weight: 186,
                value: None,
                left: Some(Box::new(HuffmanNode {
                    weight: 79,
                    value: None,
                    left: Some(Box::new(HuffmanNode {
                        weight: 37,
                        value: Some('u'),
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(HuffmanNode {
                        weight: 42,
                        value: Some('d'),
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(HuffmanNode {
                    weight: 107,
                    value: None,
                    left: Some(Box::new(HuffmanNode {
                        weight: 42,
                        value: Some('l'),
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(HuffmanNode {
                        weight: 65,
                        value: None,
                        left: Some(Box::new(HuffmanNode {
                            weight: 32,
                            value: Some('c'),
                            left: None,
                            right: None,
                        })),
                        right: Some(Box::new(HuffmanNode {
                            weight: 33,
                            value: None,
                            left: Some(Box::new(HuffmanNode {
                                weight: 9,
                                value: None,
                                left: Some(Box::new(HuffmanNode {
                                    weight: 2,
                                    value: Some('z'),
                                    left: None,
                                    right: None,
                                })),
                                right: Some(Box::new(HuffmanNode {
                                    weight: 7,
                                    value: Some('k'),
                                    left: None,
                                    right: None,
                                })),
                            })),
                            right: Some(Box::new(HuffmanNode {
                                weight: 24,
                                value: Some('m'),
                                left: None,
                                right: None,
                            })),
                        })),
                    })),
                })),
            })),
        });

        assert_eq!(build_huffman_tree(&freqs), Some(expected));
    }

    #[test]
    fn test_assign_huffman_codes_webexample() {
        let huffman_tree = Box::new(HuffmanNode {
            weight: 306,
            value: None,
            left: Some(Box::new(HuffmanNode {
                weight: 120,
                value: Some('e'),
                left: None,
                right: None,
            })),
            right: Some(Box::new(HuffmanNode {
                weight: 186,
                value: None,
                left: Some(Box::new(HuffmanNode {
                    weight: 79,
                    value: None,
                    left: Some(Box::new(HuffmanNode {
                        weight: 37,
                        value: Some('u'),
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(HuffmanNode {
                        weight: 42,
                        value: Some('d'),
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(HuffmanNode {
                    weight: 107,
                    value: None,
                    left: Some(Box::new(HuffmanNode {
                        weight: 42,
                        value: Some('l'),
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(HuffmanNode {
                        weight: 65,
                        value: None,
                        left: Some(Box::new(HuffmanNode {
                            weight: 32,
                            value: Some('c'),
                            left: None,
                            right: None,
                        })),
                        right: Some(Box::new(HuffmanNode {
                            weight: 33,
                            value: None,
                            left: Some(Box::new(HuffmanNode {
                                weight: 9,
                                value: None,
                                left: Some(Box::new(HuffmanNode {
                                    weight: 2,
                                    value: Some('z'),
                                    left: None,
                                    right: None,
                                })),
                                right: Some(Box::new(HuffmanNode {
                                    weight: 7,
                                    value: Some('k'),
                                    left: None,
                                    right: None,
                                })),
                            })),
                            right: Some(Box::new(HuffmanNode {
                                weight: 24,
                                value: Some('m'),
                                left: None,
                                right: None,
                            })),
                        })),
                    })),
                })),
            })),
        });

        let mut expected: Vec<HuffmanCode> = vec![];

        expected.push(HuffmanCode {
            value: 'c',
            freq: 32,
            code: String::from("1110"),
        });
        expected.push(HuffmanCode {
            value: 'd',
            freq: 42,
            code: String::from("101"),
        });
        expected.push(HuffmanCode {
            value: 'e',
            freq: 120,
            code: String::from("0"),
        });
        expected.push(HuffmanCode {
            value: 'k',
            freq: 7,
            code: String::from("111101"),
        });
        expected.push(HuffmanCode {
            value: 'l',
            freq: 42,
            code: String::from("110"),
        });
        expected.push(HuffmanCode {
            value: 'm',
            freq: 24,
            code: String::from("11111"),
        });
        expected.push(HuffmanCode {
            value: 'u',
            freq: 37,
            code: String::from("100"),
        });
        expected.push(HuffmanCode {
            value: 'z',
            freq: 2,
            code: String::from("111100"),
        });

        expected.sort_by(|a, b| a.value.cmp(&b.value));

        let mut actual = generate_huffman_codes(&huffman_tree);
        actual.sort_by(|a, b| a.value.cmp(&b.value));

        assert_eq!(actual, expected)
    }
}
