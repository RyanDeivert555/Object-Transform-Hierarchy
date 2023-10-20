#![allow(unused)]
use raylib::prelude::*;
use std::collections::HashSet;
use uuid::Uuid;

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

    pub fn parent(&self) -> Uuid {
        self.parent
    }

    pub fn children(&self) -> &HashSet<Uuid> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut HashSet<Uuid> {
        &mut self.children
    }

    pub fn set_parent(&mut self, new_parent: Uuid) {
        self.parent = new_parent;
    }

    pub fn add_child(&mut self, child: Uuid) {
        self.children.insert(child);
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn unset_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn orientation(&self) -> Quaternion {
        self.orientation
    }

    pub fn euler_angle(&self) -> Vector3 {
        self.orientation.to_euler()
    }

    pub fn world_matrix(&self) -> Matrix {
        self.world_matrix
    }

    pub fn gl_world_matrix(&self) -> Matrix {
        self.gl_world_matrix
    }

    pub fn depth_vector(&self) -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0).transform_with(self.orientation.to_matrix().inverted())
    }

    pub fn vertical_vector(&self) -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0).transform_with(self.orientation.to_matrix().inverted())
    }

    pub fn horizontal_negated_vector(&self) -> Vector3 {
        self.vertical_vector().cross(self.depth_vector())
    }

    pub fn horizonal_post_vector(&self) -> Vector3 {
        self.depth_vector().cross(self.vertical_vector())
    }

    pub fn world_position(&self) -> Vector3 {
        Vector3::zero().transform_with(self.world_matrix)
    }

    pub fn set_position(&mut self, new_postion: Vector3) {
        self.position = new_postion;
        self.set_dirty();
    }

    pub fn set_orientation(&mut self, new_angle: Vector3) {
        let angle = new_angle.scale_by(DEG2RAD as f32);
        self.orientation = Quaternion::from_euler(angle.x, angle.y, angle.z);
        self.set_dirty();
    }

    pub fn set_world_matrix(&mut self, matrix: Matrix) {
        self.world_matrix = matrix;
    }

    pub fn set_gl_world_matrix(&mut self, matrix: Matrix) {
        self.gl_world_matrix = matrix;
    }

    pub fn move_by(&mut self, offset: Vector3) {
        self.position += offset;
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn look_at(&mut self, target: Vector3, up: Vector3) {
        self.set_dirty();
        let matrix = Matrix::look_at(self.position, target, up);
        self.orientation = Quaternion::from_matrix(matrix);
    }

    pub fn local_matrix(&self) -> Matrix {
        let matrix_orientation = Quaternion::to_matrix(&self.orientation);
        let translation = Matrix::translate(self.position.x, self.position.y, self.position.z);
        matrix_orientation.inverted() * translation
    }

    // ALL MOVEMENT MUST BE DONE THOUGH MAP SO CHILDREN ARE MARKED AS DIRTY
    pub fn move_vertical(&mut self, distance: f32) {
        self.set_dirty();
        self.position += self.vertical_vector().scale_by(distance);
    }

    pub fn move_depth(&mut self, distance: f32) {
        self.set_dirty();
        self.position += self.depth_vector().scale_by(distance);
    }

    pub fn move_horizontal(&mut self, distance: f32) {
        self.set_dirty();
        self.position += self.horizontal_negated_vector().scale_by(distance);
    }

    pub fn rotate_vertical(&mut self, angle: f32) {
        self.set_dirty();
        let rotation = Quaternion::from_euler(0.0, -angle * DEG2RAD as f32, 0.0);
        self.orientation = rotation * self.orientation;
    }

    pub fn rotate_depth(&mut self, angle: f32) {
        self.set_dirty();
        let rotation = Quaternion::from_euler(0.0, 0.0, -angle * DEG2RAD as f32);
        self.orientation = rotation * self.orientation;
    }

    pub fn rotate_horizontal(&mut self, angle: f32) {
        self.set_dirty();
        let rotation = Quaternion::from_euler(angle * DEG2RAD as f32, 0.0, 0.0);
        self.orientation = rotation * self.orientation;
    }
}
