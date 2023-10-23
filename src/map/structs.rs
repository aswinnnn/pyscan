struct DependencyGraph {
    inner: Vec<DependencyNode>
}

struct DependencyNode {
    inner: String,
    version: String,
    node: Vec<DependencyNode>
}

impl DependencyGraph {
    // ... your existing methods ...

    fn display_ascii_tree(&self) {
        self.display_ascii_tree_recursive(&self.inner, &mut " ".to_string(), if self.inner.len() > 1 { false } else { true });
    }

    fn display_ascii_tree_recursive(&self, nodes: &Vec<DependencyNode>, pad: &mut String, is_child: bool) {
        if nodes.is_empty() {
            return;
        }

        let parent = "|->";
        let mut new_pad = pad.clone();
        if is_child {
            new_pad.push_str(". ");
        }

        for (i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let connector = if is_last { ">  " } else { "^-- " };
            println!("{}{}{} {}", new_pad, if i == 0 { parent } else { connector }, node.inner, node.version);
            self.display_ascii_tree_recursive(&node.node, &mut new_pad, true); // Pass true for is_child
        }
    }
}
