#![allow(unused)]
use crate::object_transform::ObjectTransform;
use raylib::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use uuid::Uuid;

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
        self.add_transform(transform)
    }

    pub fn add_transform(&mut self, transform: ObjectTransform) -> Uuid {
        let id = Uuid::new_v4();
        self.transforms.insert(id, transform);
        id
    }

    pub fn parent(&self, id: Uuid) -> Uuid {
        self.transforms.get(&id).unwrap().parent()
    }

    pub fn childern(&self, id: Uuid) -> &HashSet<Uuid> {
        self.transforms.get(&id).unwrap().children()
    }

    // no getter methods, all must be handled through map
    pub fn get(&self, id: Uuid) -> &ObjectTransform {
        self.transforms.get(&id).unwrap()
    }

    pub fn get_mut(&mut self, id: Uuid) -> &mut ObjectTransform {
        self.transforms.get_mut(&id).unwrap()
    }

    pub fn entry(&mut self, id: Uuid) -> Entry<Uuid, ObjectTransform> {
        self.transforms.entry(id)
    }

    pub fn add_child(&mut self, parent: Uuid, mut child: ObjectTransform) -> Uuid {
        // set child transform's parent
        child.set_parent(parent);
        // set parent transform id with children
        let child_id = self.add_transform(child);
        let parent_transform = self.get_mut(parent);
        parent_transform.add_child(child_id);
        // set all children transform to have new parent
        let children = self.childern(child_id);
        for child in children.to_owned() {
            let child_transform = self.get_mut(child);
            child_transform.set_parent(parent);
        }
        child_id
    }

    pub fn add_child_id(&mut self, parent: Uuid, child: Uuid) {
        {
            let parent_transform = self.transforms.get_mut(&parent).unwrap();
            parent_transform.add_child(child);
        }
        {
            let child_transform = self.transforms.get_mut(&child).unwrap();
            child_transform.set_parent(parent);
        }
        let children = self.childern(child);
        for child in children.to_owned() {
            let transform = self.transforms.get_mut(&child).unwrap();
            transform.set_parent(parent);
        }
    }

    fn set_dirty(&mut self, id: Uuid) {
        self.entry(id).and_modify(|o| o.set_dirty());
        // ok to clone ids, they are cheap
        for child in self.childern(id).to_owned().into_iter() {
            self.set_dirty(child);
        }
    }

    pub fn update_world_matrix(&mut self, id: Uuid) {
        let parent_id = self.parent(id);
        let parent_transform = self.get_mut(parent_id);
        if !parent_transform.dirty() {
            return;
        }
        let parent_matrix = if parent_id == id {
            Matrix::identity()
        } else {
            parent_transform.world_matrix()
        };

        parent_transform.set_world_matrix(parent_transform.local_matrix() * parent_matrix);
        parent_transform.set_gl_world_matrix(parent_transform.gl_world_matrix().transposed());
        parent_transform.unset_dirty();
    }

    pub fn world_matrix(&mut self, id: Uuid) -> Matrix {
        self.update_world_matrix(id);
        let transform = self.get(id);
        transform.world_matrix()
    }

    pub fn gl_world_matrix(&mut self, id: Uuid) -> Matrix {
        self.update_world_matrix(id);
        let transform = self.get(id);
        transform.gl_world_matrix()
    }

    pub fn set_camera(&mut self, id: Uuid, camera: &mut Camera) {
        let transform = self.get(id);
        let world_matrix = transform.world_matrix();
        camera.position = Vector3::zero().transform_with(world_matrix);
        camera.target = Vector3::new(0.0, 0.0, 1.0).transform_with(world_matrix);
        camera.up = Vector3::new(0.0, 1.0, 0.0).transform_with(world_matrix) - camera.target;
    }
}
