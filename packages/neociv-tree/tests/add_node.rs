use neociv_tree::tree::NeocivTree;

#[test]
pub fn add_node() {
    let key = String::from("example");
    let mut tree = NeocivTree::new("example-tree".to_string());
    assert!(tree.add(key.clone(), None, None, None, None, None).is_ok());
    assert!(tree.contains(&key));
    assert_eq!(tree.get(&key).unwrap().col, 0);
    assert_eq!(tree.get(&key).unwrap().row, 0);
}
