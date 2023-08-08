use neociv_tree::tree::NeocivTree;

#[test]
pub fn add_node() {
    let mut tree = NeocivTree::new("example".to_string());
    assert!(tree.add("example".to_string(), None, None, None, None, None).is_ok());
    assert!(tree.contains(&"example".to_string()));
}
