pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;

    #[derive(Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::default(),
                edges: Vec::default(),
                attrs: HashMap::default(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.append(&mut nodes.to_owned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.append(&mut edges.to_owned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == node_name)
        }
    }

    impl Default for Graph {
        fn default() -> Self {
            Self::new()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub attrs: HashMap<String, String>,
                pub src: String,
                pub dst: String,
            }

            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Self {
                        attrs: HashMap::default(),
                        src: src.to_string(),
                        dst: dst.to_string(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(&attr.to_string()).map(|x| x.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub attrs: HashMap<String, String>,
                pub name: String,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(&attr_name.to_string()).map(|x| x.as_str())
                }
            }
        }
    }
}
