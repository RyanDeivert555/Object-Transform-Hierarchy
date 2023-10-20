#[cfg(test)]
mod tests {
    use crate::transform_map::TransformMap;

    #[test]
    fn creation() {
        let mut transform_map = TransformMap::new();
        let i1 = transform_map.new_transform(true);
        let i2 = transform_map.new_transform(true);

        assert_ne!(i1, i2);
    }
}
