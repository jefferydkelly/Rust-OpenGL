use std::ops::Index;

use nalgebra_glm::vec3;
use rapier3d::prelude::*;
use glm::{Vec3};
use once_cell::sync::OnceCell;

pub struct PhysicsManager {
    rigid_bodies:RigidBodySet,
    colliders:ColliderSet,
    joints:JointSet,
    islands:IslandManager,
    broad_phase:BroadPhase,
    narrow_phase:NarrowPhase,
    parameters:IntegrationParameters,
    ccd_solver:CCDSolver,
    gravity:Vec3,
    event_handler:(),
    physics_hooks:(),
    pipeline:PhysicsPipeline

}


static mut PHYSICS_MANAGER:OnceCell<PhysicsManager> = OnceCell::new();

impl PhysicsManager {
    pub fn create_instance() {
        
    
        //Setup the physics engine
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
        collider_set.insert(collider);

        let rigid_body = RigidBodyBuilder::new_dynamic().translation(vector![0.0, 10.0, 0.0]).build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut &mut rigid_body_set);

        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut joint_set = JointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        let pm = PhysicsManager {
            rigid_bodies:rigid_body_set,
            colliders:collider_set,
            joints:joint_set,
            islands:island_manager,
            parameters:integration_parameters,
            broad_phase:broad_phase,
            narrow_phase:narrow_phase,
            ccd_solver:ccd_solver,
            gravity:gravity,
            physics_hooks:physics_hooks,
            event_handler:event_handler,
            pipeline:physics_pipeline
        };

        unsafe {
            PHYSICS_MANAGER.set(pm);
        }
    
    }

    pub fn get_instance()->&'static mut PhysicsManager {
        unsafe  {
            PHYSICS_MANAGER.get_mut().expect("Input Manager has not been created")
        }
    }

    pub fn step(&mut self, dt:f32) {
        self.parameters.dt = dt;
        self.pipeline.step(
            &self.gravity, 
            &self.parameters, 
            &mut self.islands, 
            &mut self.broad_phase, 
            &mut self.narrow_phase, 
            &mut &mut self.rigid_bodies, 
            &mut self.colliders, 
            &mut self.joints, 
            &mut self.ccd_solver, 
            &self.physics_hooks, 
            &self.event_handler
        );
    }

    pub fn create_dynamic_body(&mut self) -> RigidBodyHandle {
        let body_builder = RigidBodyBuilder::new_dynamic()
        .gravity_scale(1.0)
        .can_sleep(true)
        .ccd_enabled(true);
        let body = body_builder.build();

        let handle = self.rigid_bodies.insert(body);
        handle
    }

    pub fn create_static_body(&mut self) -> RigidBodyHandle {
        let body_builder = RigidBodyBuilder::new_static().ccd_enabled(true);
        let body = body_builder.build();

        let handle = self.rigid_bodies.insert(body);
        handle
    }

    pub fn get_rigid_body(&mut self, handle:RigidBodyHandle)->&mut RigidBody {
        return self.rigid_bodies.get_mut(handle).unwrap();
    }

    pub fn create_cuboid_collider(&mut self, position:Vec3, size:Vec3, parent_handle:RigidBodyHandle)-> ColliderHandle {
        let builder = ColliderBuilder::cuboid(size.x, size.y, size.z).translation(position);
        let collider = builder.build();
        let handle = self.colliders.insert_with_parent(collider, parent_handle, &mut self.rigid_bodies);
        handle
    }
}