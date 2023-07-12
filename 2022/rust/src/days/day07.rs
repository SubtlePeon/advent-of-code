use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u32>,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

pub fn part_one(input: &str) -> u32 {
    let mut tree = HashMap::new();
    let mut curr = String::from("");

    for line in input.lines() {
        if line == "$ cd /" {
            curr = String::from("/");
            tree.insert(
                String::from("/"),
                Node {
                    name: String::from("/"),
                    is_dir: true,
                    size: None,
                    children: Vec::new(),
                    parent: None,
                },
            );
        } else if line == "$ cd .." {
            println!("{}", line);
            println!("From dir {} to parent {:?}", curr, tree.get(&curr).unwrap().parent);
            let len = curr.len();
            let idx = curr[..len - 1].rfind("/").unwrap();
            curr = String::from(&curr[..=idx]);
        } else if line.starts_with("$ cd ") {
            println!("{}", line);
            println!(
                "From dir {} with children {:?}, go to {}",
                &curr,
                tree.get(&curr).unwrap().children,
                &line[5..],
            );
            // Check that curr is in the children of the parent
            let child_name = curr.clone() + &line[5..] + "/";
            assert!(tree.get(&curr).unwrap().children.contains(&child_name));

            curr = child_name;
        } else if line == "$ ls" {
            // do nothing
        } else if line.starts_with("dir ") {
            let child_name = curr.clone() + &line[4..] + "/";
            // Register as a child of current
            tree.get_mut(&curr).unwrap().children.push(child_name.clone());
            // Put in hashmap
            tree.insert(
                child_name,
                Node {
                    name: String::from(&line[4..]),
                    is_dir: true,
                    size: None,
                    children: Vec::new(),
                    parent: Some(String::from(&curr)),
                }
            );
        } else {
            // This is a sized file
            let mut data = line.split(" ");
            let size = data.next().unwrap().parse::<u32>().unwrap();
            let name = data.next().unwrap();
            let name = curr.clone() + &name;

            // Register as a child of current
            tree.get_mut(&curr).unwrap().children.push(name.clone());
            // Register in hashmap
            tree.insert(
                name.clone(),
                Node {
                    name,
                    is_dir: false,
                    size: Some(size),
                    children: Vec::new(),
                    parent: Some(String::from(&curr)),
                }
            );
        }
    }
    

    // Find stuff
    let mut total = 0;
    fn add_size(total: &mut u32, tree: &HashMap<String, Node>, dir: String) -> u32 {
        let mut size = 0;
        for child_key in tree.get(&dir).unwrap().children.iter() {
            let child = tree.get(child_key).unwrap();
            if child.is_dir {
                size += add_size(total, tree, String::from(child_key));
            } else {
                size += child.size.unwrap();
            }
        }
        if size <= 100_000 {
            *total += size;
        }
        size
    }
    add_size(&mut total, &tree, String::from("/"));
    total
}
