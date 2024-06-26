use rapier2d::prelude::*;
use salva2d::object::*;
use salva2d::solver::Akinci2013SurfaceTension;
use salva2d::solver::ArtificialViscosity;
use salva2d::solver::Becker2009Elasticity;
use salva2d::solver::DFSPHViscosity;
use salva2d::solver::He2014SurfaceTension;
use salva2d::solver::WCSPHSurfaceTension;
use salva2d::solver::XSPHViscosity;
use salva2d::math::Vector as SalvaVector;
use crate::convert::*;
use crate::handle::*;
use crate::vector::Vector;
use crate::physics_world::*;
use crate::shape::*;

pub fn pixel_vector_array_to_vec(pixel_data : &Vector, data_count : usize) -> Vec<SalvaVector<Real>> {
    let mut vec = Vec::<SalvaVector::<Real>>::with_capacity(data_count);
    unsafe {
        let data_raw = std::slice::from_raw_parts(pixel_data, data_count);
        for pixel_point in data_raw {
            let point = &vector_pixels_to_meters(pixel_point);
            let salva_vector = SalvaVector::<Real>::new(point.x, point.y);
            vec.push(salva_vector);
        }
    }
    return vec;
}

#[no_mangle]
pub extern "C" fn fluid_create(world_handle : Handle, density: Real) -> HandleDouble {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let particle_radius = physics_world.fluids_pipeline.liquid_world.particle_radius();
    let fluid = Fluid::new(Vec::new(), particle_radius, density);
	return fluid_handle_to_handle(physics_world.fluids_pipeline.liquid_world.add_fluid(fluid));
}

#[no_mangle]
pub extern "C" fn fluid_change_density(world_handle : Handle, fluid_handle: HandleDouble, density: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    fluid.density0 = density;

}

#[no_mangle]
pub extern "C" fn fluid_change_points_and_velocities(world_handle : Handle, fluid_handle: HandleDouble, pixel_points : &Vector, point_count : usize, velocity_points : &Vector) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let points = pixel_point_array_to_vec(pixel_points, point_count);
    let velocities = pixel_vector_array_to_vec(velocity_points, point_count);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let mut accelerations: Vec<_> = std::iter::repeat(salva2d::math::Vector::zeros())
        .take(point_count)
        .collect();
    fluid.positions = points;
    // copy back the accelerations that were before, if they exist
    for i in 0..point_count {
        if fluid.accelerations.len() > i {
            accelerations[i] = fluid.accelerations[i];
        }
    }
    fluid.velocities = velocities;
    fluid.accelerations = accelerations;
    fluid.volumes = std::iter::repeat(fluid.default_particle_volume())
    .take(point_count)
    .collect();
}

#[no_mangle]
pub extern "C" fn fluid_change_points(world_handle : Handle, fluid_handle: HandleDouble, pixel_points : &Vector, point_count : usize) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let points = pixel_point_array_to_vec(pixel_points, point_count);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let mut velocities: Vec<_> = std::iter::repeat(salva2d::math::Vector::zeros())
        .take(point_count)
        .collect();
    let mut accelerations: Vec<_> = std::iter::repeat(salva2d::math::Vector::zeros())
        .take(point_count)
        .collect();
    fluid.positions = points;
    // copy back the velocities and accelerations that were before, if they exist
    for i in 0..point_count {
        if fluid.velocities.len() > i {
            velocities[i] = fluid.velocities[i];
        }
        if fluid.accelerations.len() > i {
            accelerations[i] = fluid.accelerations[i];
        }
    }
    fluid.velocities = velocities;
    fluid.accelerations = accelerations;
    fluid.volumes = std::iter::repeat(fluid.default_particle_volume())
    .take(point_count)
    .collect();
}

#[no_mangle]
pub extern "C" fn fluid_delete_points(world_handle : Handle, fluid_handle: HandleDouble, indexes : &usize, indexes_count : usize) {
    let mut physics_engine = singleton().lock().unwrap();
    let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    unsafe {
        let indexes_raw = std::slice::from_raw_parts(indexes, indexes_count);
        for i in 0..indexes_raw.len() {
            let index_raw = indexes_raw[i];
            if fluid.positions.len() <= index_raw {
                continue;
            }
            fluid.positions.remove(index_raw);
            fluid.velocities.remove(index_raw);
            fluid.accelerations.remove(index_raw);
            fluid.volumes.remove(index_raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn fluid_add_points_and_velocities(world_handle : Handle, fluid_handle: HandleDouble, pixel_points : &Vector, point_count : usize, velocity_points : &Vector) {
    let mut physics_engine = singleton().lock().unwrap();
    let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let points = pixel_point_array_to_vec(pixel_points, point_count);
    let velocities = pixel_vector_array_to_vec(velocity_points, point_count);
    // add old positions from fluid
    fluid.positions.extend_from_slice(&points);
    fluid.velocities.extend_from_slice(&velocities);
    let new_point_count = fluid.positions.len();
    let mut accelerations: Vec<_> = std::iter::repeat(salva2d::math::Vector::zeros())
        .take(new_point_count)
        .collect();
    // copy back the accelerations that were before, if they exist
    for i in 0..fluid.accelerations.len() {
        accelerations[i] = fluid.accelerations[i];
    }
    fluid.accelerations = accelerations;
    fluid.volumes = std::iter::repeat(fluid.default_particle_volume())
    .take(new_point_count)
    .collect();
}

#[no_mangle]
pub extern "C" fn fluid_get_points(world_handle : Handle, fluid_handle: HandleDouble, pixel_points : &mut Vector, point_count : usize) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    unsafe {
        let points_raw = std::slice::from_raw_parts_mut(pixel_points, point_count);
        for i in 0..points_raw.len() {
            if fluid.positions.len() <= i {
                return;
            }
            points_raw[i].x = meters_to_pixels(fluid.positions[i].x);
            points_raw[i].y = meters_to_pixels(fluid.positions[i].y);
        }
    }
}

#[no_mangle]
pub extern "C" fn fluid_get_velocities(world_handle : Handle, fluid_handle: HandleDouble, pixel_velocities : &mut Vector, velocity_count : usize) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    unsafe {
        let velocity_raw = std::slice::from_raw_parts_mut(pixel_velocities, velocity_count);
        for i in 0..velocity_raw.len() {
            if fluid.velocities.len() <= i {
                return;
            }
            velocity_raw[i].x = meters_to_pixels(fluid.velocities[i].x);
            velocity_raw[i].y = meters_to_pixels(fluid.velocities[i].y);
        }
    }
}

#[no_mangle]
pub extern "C" fn fluid_get_accelerations(world_handle : Handle, fluid_handle: HandleDouble, pixel_acceleration : &mut Vector, acceleration_count : usize) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    unsafe {
        let acceleration_raw = std::slice::from_raw_parts_mut(pixel_acceleration, acceleration_count);
        for i in 0..acceleration_raw.len() {
            if fluid.accelerations.len() <= i {
                return;
            }
            acceleration_raw[i].x = meters_to_pixels(fluid.accelerations[i].x);
            acceleration_raw[i].y = meters_to_pixels(fluid.accelerations[i].y);
        }
    }
}

#[no_mangle]
pub extern "C" fn fluid_clear_effects(world_handle : Handle, fluid_handle: HandleDouble) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    fluid.nonpressure_forces.clear();
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_elasticity(world_handle : Handle, fluid_handle: HandleDouble, young_modulus: Real, poisson_ratio: Real, nonlinear_strain: bool) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: Becker2009Elasticity = Becker2009Elasticity::new(young_modulus, poisson_ratio, nonlinear_strain);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_surface_tension_akinci(world_handle : Handle, fluid_handle: HandleDouble, fluid_tension_coefficient: Real, boundary_adhesion_coefficient: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: Akinci2013SurfaceTension = Akinci2013SurfaceTension::new(fluid_tension_coefficient, boundary_adhesion_coefficient);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_surface_tension_he(world_handle : Handle, fluid_handle: HandleDouble, fluid_tension_coefficient: Real, boundary_adhesion_coefficient: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: He2014SurfaceTension = He2014SurfaceTension::new(fluid_tension_coefficient, boundary_adhesion_coefficient);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_surface_tension_wcsph(world_handle : Handle, fluid_handle: HandleDouble, fluid_tension_coefficient: Real, boundary_adhesion_coefficient: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: WCSPHSurfaceTension = WCSPHSurfaceTension::new(fluid_tension_coefficient, boundary_adhesion_coefficient);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_viscosity_artificial(world_handle : Handle, fluid_handle: HandleDouble, fluid_viscosity_coefficient: Real, boundary_viscosity_coefficient: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: ArtificialViscosity = ArtificialViscosity::new(fluid_viscosity_coefficient, boundary_viscosity_coefficient);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_viscosity_dfsph(world_handle : Handle, fluid_handle: HandleDouble, fluid_viscosity_coefficient: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: DFSPHViscosity = DFSPHViscosity::new(fluid_viscosity_coefficient);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_add_effect_viscosity_xsph(world_handle : Handle, fluid_handle: HandleDouble, fluid_viscosity_coefficient: Real, boundary_viscosity_coefficient: Real) {
	let mut physics_engine = singleton().lock().unwrap();
	let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    let fluid = fluid.unwrap();
    let effect: XSPHViscosity = XSPHViscosity::new(fluid_viscosity_coefficient, boundary_viscosity_coefficient);
    fluid.nonpressure_forces.push(Box::new(effect));
}

#[no_mangle]
pub extern "C" fn fluid_destroy(world_handle : Handle, fluid_handle: HandleDouble) {
    let mut physics_engine = singleton().lock().unwrap();
    let physics_world = physics_engine.get_world(world_handle);
    let fluid = physics_world.fluids_pipeline.liquid_world.fluids_mut().get_mut(handle_to_fluid_handle(fluid_handle));
    assert!(fluid.is_some());
    // TODO reenable after the function is fixed https://github.com/dimforge/salva/pull/37/files
    //physics_world.fluids_pipeline.liquid_world.remove_fluid(handle_to_fluid_handle(fluid_handle));
}
