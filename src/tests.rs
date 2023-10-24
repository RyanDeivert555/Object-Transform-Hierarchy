#[cfg(test)]
mod tests {
    use crate::transform_map::TransformMap;

    #[test]
    fn creation() {
        let mut transform_map = TransformMap::new();
        for _ in 0..10_000 {
            let i1 = transform_map.new_transform(true);
            let i2 = transform_map.new_transform(true);

            assert_ne!(i1, i2);
        }
    }

    #[test]
    fn hierarchy() {
        let mut transform_map = TransformMap::new();
        let parent = transform_map.new_transform(true);

        let child1 = transform_map.new_transform(true);
        transform_map.add_child_from_id(parent, child1);
        let child2 = transform_map.add_child(parent, true);

        assert_eq!(transform_map.children_count(parent), 2);
        assert!(transform_map.is_parent(parent));
        assert!(transform_map.has_child(parent, child1));
        assert!(transform_map.has_child(parent, child2));
        assert!(transform_map.is_parent_of(parent, child1));
        assert!(transform_map.is_parent_of(parent, child2));

        // child1 is parent now
        transform_map.reparent(parent, Some(child1));
        assert!(transform_map.is_parent(child1));
        assert!(!transform_map.is_parent(parent));
        assert!(transform_map.has_child(child1, parent));
        // child2 is still a child of parent
        assert!(!transform_map.is_parent_of(child1, child2));
        assert!(transform_map.is_parent_of(parent, child2));
        assert!(!transform_map.has_child(child1, child2));
    }
}
