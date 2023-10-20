#![allow(unused)]
use raylib::prelude::*;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// HACK: make struct fields public to only TransformMap
#[derive(Debug, Default)]
pub struct ObjectTransform {
    position: Vector3,
    orientation: Quaternion,
    dirty: bool,
    world_matrix: Matrix,
    gl_world_matrix: Matrix,
    parent: Uuid,
    children: HashSet<Uuid>,
}

impl ObjectTransform {
    pub fn new(face_y: bool) -> Self {
        let position = Vector3::zero();
        let orientation = if face_y {
            Quaternion::from_axis_angle(Vector3::new(0.0, 1.0, 0.0), 0.0)
        } else {
            Quaternion::identity()
        };
        let dirty = true;
        let world_matrix = Matrix::zero();
        let gl_world_matrix = Matrix::zero();

        Self {
            position,
            orientation,
            dirty,
            world_matrix,
            gl_world_matrix,
            parent: Uuid::new_v4(),
            children: HashSet::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TransformMap {
    transforms: HashMap<Uuid, ObjectTransform>,
}

impl TransformMap {
    // TODO: implement reparent, detatch
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_transform(&mut self, face_y: bool) -> Uuid {
        let transform = ObjectTransform::new(face_y);
        let id = transform.parent;
        self.transforms.insert(id, transform);
        id
    }

    fn get(&self, id: Uuid) -> &ObjectTransform {
        self.transforms.get(&id).unwrap()
    }

    fn get_mut(&mut self, id: Uuid) -> &mut ObjectTransform {
        self.transforms.get_mut(&id).unwrap()
    }

    pub fn reparent(&mut self, old_parent: Uuid, new_parent: Uuid) {
        if old_parent == new_parent {
            return;
        }
        {
            let new_parent_transform = self.get_mut(new_parent);
            new_parent_transform.parent = new_parent;
            new_parent_transform.children.insert(old_parent);
        }
        {
            let old_parent_transform = self.get_mut(old_parent);
            old_parent_transform.parent = new_parent;
            old_parent_transform.children.remove(&new_parent);
        }
    }

    pub fn add_child(&mut self, parent: Uuid, face_y: bool) -> Uuid {
        let child = self.new_transform(face_y);
        self.add_child_from_id(parent, child);
        child
    }

    pub fn add_child_from_id(&mut self, parent: Uuid, child: Uuid) {
        {
            let parent_transform = self.get_mut(parent);
            parent_transform.children.insert(child);
        }
        {
            let child_transform = self.get_mut(child);
            child_transform.parent = parent;
        }
    }

    // TODO: detach

    pub fn set_dirty(&mut self, id: Uuid) {
        self.get_mut(id).dirty = true;
        let transform = self.get(id);
        // cheap clone?
        for child in transform.children.to_owned().iter() {
            self.set_dirty(*child);
        }
    }

    pub fn position(&self, id: Uuid) -> Vector3 {
        self.get(id).position
    }

    pub fn orientation(&self, id: Uuid) -> Quaternion {
        self.get(id).orientation
    }

    pub fn euler_angle(&self, id: Uuid) -> Vector3 {
        self.orientation(id).to_euler()
    }

    pub fn depth_vector(&self, id: Uuid) -> Vector3 {
        let transform = self.get(id);
        Vector3::new(0.0, 0.0, 1.0).transform_with(transform.orientation.to_matrix().inverted())
    }

    pub fn vertical_vector(&self, id: Uuid) -> Vector3 {
        let transform = self.get(id);
        Vector3::new(0.0, 1.0, 0.0).transform_with(transform.orientation.to_matrix().inverted())
    }

    pub fn horizontal_negated_vector(&self, id: Uuid) -> Vector3 {
        self.vertical_vector(id).cross(self.depth_vector(id))
    }

    pub fn horizontal_post_vector(&self, id: Uuid) -> Vector3 {
        self.depth_vector(id).cross(self.vertical_vector(id))
    }

    pub fn world_position(&mut self, id: Uuid) -> Vector3 {
        Vector3::zero().transform_with(self.world_matrix(id))
    }

    pub fn world_target_position(&mut self, id: Uuid) -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0).transform_with(self.world_matrix(id))
    }

    pub fn move_by(&mut self, id: Uuid, offset: Vector3) {
        self.set_dirty(id);
        let transform = self.get_mut(id);
        transform.position += offset;
    }

    pub fn set_position(&mut self, id: Uuid, position: Vector3) {
        self.set_dirty(id);
        let transform = self.get_mut(id);
        transform.position = position;
    }

    pub fn set_orientation(&mut self, id: Uuid, euler_angle: Vector3) {
        self.set_dirty(id);
        let angle = euler_angle.scale_by(DEG2RAD as f32);
        let transform = self.get_mut(id);
        transform.orientation = Quaternion::from_euler(angle.x, angle.y, angle.z);
    }

    pub fn look_at(&mut self, id: Uuid, target: Vector3, up: Vector3) {
        self.set_dirty(id);
        let transform = self.get_mut(id);
        transform.orientation =
            Quaternion::from_matrix(Matrix::look_at(transform.position, target, up));
    }

    pub fn local_matrix(&self, id: Uuid) -> Matrix {
        let transform = self.get(id);
        let orientation_matrix = transform.orientation.to_matrix();
        let translation_matrix = Matrix::translate(
            transform.position.x,
            transform.position.x,
            transform.position.x,
        );
        orientation_matrix.inverted() * translation_matrix
    }

    pub fn update_world_matrix(&mut self, id: Uuid) {
        let local_matrix = self.local_matrix(id);
        let parent_matrix = self.world_matrix(id);
        let transform = self.get_mut(id);

        transform.world_matrix = local_matrix * parent_matrix;
        transform.gl_world_matrix = transform.world_matrix.transposed();

        transform.dirty = false;
    }

    pub fn world_matrix(&mut self, id: Uuid) -> Matrix {
        let dirty = {
            let transform = self.get_mut(id);
            transform.dirty
        };
        if !dirty {
            self.get(id).world_matrix
        } else {
            self.update_world_matrix(id);
            self.get(id).world_matrix
        }
    }

    pub fn gl_world_matrix(&mut self, id: Uuid) -> Matrix {
        let dirty = {
            let transform = self.get_mut(id);
            transform.dirty
        };
        if !dirty {
            self.get(id).gl_world_matrix
        } else {
            self.update_world_matrix(id);
            self.get(id).gl_world_matrix
        }
    }

    pub fn to_local_position(&mut self, id: Uuid, in_position: Vector3) -> Vector3 {
        in_position.transform_with(self.world_matrix(id).inverted())
    }

    pub fn move_depth(&mut self, id: Uuid, distance: f32) {
        self.set_dirty(id);
        let depth_vector = self.depth_vector(id);
        let transform = self.get_mut(id);
        transform.position += depth_vector * distance;
    }

    pub fn move_vertical(&mut self, id: Uuid, distance: f32) {
        self.set_dirty(id);
        let vertical_vector = self.vertical_vector(id);
        let transform = self.get_mut(id);
        transform.position += vertical_vector * distance;
    }

    pub fn move_horizontal(&mut self, id: Uuid, distance: f32) {
        self.set_dirty(id);
        let horizontal_vector = self.horizontal_negated_vector(id);
        let transform = self.get_mut(id);
        transform.position += horizontal_vector * distance;
    }

    pub fn rotate_depth(&mut self, id: Uuid, angle: f32) {
        self.set_dirty(id);
        let transform = self.get_mut(id);
        let rotation = Quaternion::from_euler(0.0, 0.0, -angle.to_radians());
        transform.orientation = rotation * transform.orientation;
    }

    pub fn rotate_vertical(&mut self, id: Uuid, angle: f32) {
        self.set_dirty(id);
        let transform = self.get_mut(id);
        let rotation = Quaternion::from_euler(0.0, -angle.to_radians(), 0.0);
        transform.orientation = rotation * transform.orientation;
    }

    pub fn rotate_horizontal(&mut self, id: Uuid, angle: f32) {
        self.set_dirty(id);
        let transform = self.get_mut(id);
        let rotation = Quaternion::from_euler(angle.to_radians(), 0.0, 0.0);
        transform.orientation = rotation * transform.orientation;
    }

    pub fn set_camera(&mut self, id: Uuid, camera: &mut Camera3D) {
        let world_matrix = self.get(id).world_matrix;
        camera.position = Vector3::zero().transform_with(self.world_matrix(id));
        camera.target = Vector3::new(0.0, 0.0, 1.0).transform_with(world_matrix);
        camera.up = Vector3::new(0.0, 1.0, 0.0).transform_with(world_matrix) - camera.target;
    }
    
    // for tests only, will be removed later
    pub fn children_count(&self, id: Uuid) -> usize {
        self.get(id).children.len()
    }

    pub fn is_parent(&self, id: Uuid) -> bool {
        self.get(id).parent == id
    }

    pub fn has_child(&self, parent: Uuid, child: Uuid) -> bool {
        self.get(parent).children.contains(&child)
    }

    pub fn is_parent_of(&self, parent: Uuid, child: Uuid) -> bool {
        self.get(child).parent == parent
    }
}
