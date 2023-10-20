#[cfg(test)]
mod tests {
    use crate::object_transform::ObjectTransform;
    use crate::transform_map::TransformMap;
    use raylib::prelude::*;

    #[test]
    fn map_creation() {
        let mut tranform_map = TransformMap::new();
        for _ in 0..1000 {
            let transform1 = ObjectTransform::new(true);
            let id1 = tranform_map.add_transform(transform1);
            let transform1 = ObjectTransform::new(true);
            let id2 = tranform_map.add_transform(transform1);
            assert_ne!(id1, id2);
        }
    }

    #[test]
    fn mutate_transform() {
        let mut transform1 = ObjectTransform::new(true);

        transform1.set_dirty();
        assert!(transform1.dirty());

        transform1.move_by(Vector3::new(1.0, 2.0, -11.0));
        assert_eq!(transform1.position(), Vector3::new(1.0, 2.0, -11.0));

        transform1.move_by(Vector3::new(0.0, 5.0, 5.0));
        assert_eq!(transform1.position(), Vector3::new(1.0, 7.0, -6.0));

        let mut transform_map = TransformMap::new();
        let id1 = transform_map.add_transform(transform1);
        let transform = transform_map.get(id1);
        assert_eq!(transform.position(), Vector3::new(1.0, 7.0, -6.0));
    }

    #[test]
    fn relationships() {
        let mut parent = ObjectTransform::new(true);
        parent.set_orientation(Vector3::new(0.0, 45.0, 0.0));
        parent.set_position(Vector3::one());

        let mut child = ObjectTransform::new(true);
        child.set_orientation(Vector3::new(0.0, 45.0, 0.0));
        child.move_depth(10.0);

        let mut transform_map = TransformMap::new();
        let parent_id = transform_map.add_transform(parent);
        let child_id = transform_map.add_transform(child);
        transform_map.add_child_id(parent_id, child_id);

        assert_eq!(parent_id, transform_map.get(child_id).parent());

        let parent = ObjectTransform::new(true);
        let child = ObjectTransform::new(true);
        let parent_id = transform_map.add_transform(parent);
        let child_id = transform_map.add_child(parent_id, child);

        assert_eq!(parent_id, transform_map.get(child_id).parent());
    }

    #[test]
    fn adding_children() {
        let mut transform_map = TransformMap::new();
        let parent = transform_map.add_transform(ObjectTransform::new(true));
        for _ in 0..100 {
            transform_map.add_child(parent, ObjectTransform::new(true));
        }

        assert!(transform_map.get(parent).children().len() == 100);

        for _ in 0..100 {
            let child_id = transform_map.add_transform(ObjectTransform::new(true));
            transform_map.add_child_id(parent, child_id);
        }

        assert!(transform_map.get(parent).children().len() == 200);
    }
}
