#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let tree = Tree::new();
        let root = tree.make();
        tree.set_root(&root);

        let child1 = tree.make_with( );
        tree.append(&root,&child1);

        let child2 = tree.make();
        tree.append(&root,&child2);

        assert_eq!(5, 5);
    }
}