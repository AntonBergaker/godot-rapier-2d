#include "rapier_direct_body_state_2d.h"

#include "../servers/rapier_physics_server_2d.h"
#include "../spaces/rapier_direct_space_state_2d.h"
#include "rapier_body_2d.h"

Vector2 RapierDirectBodyState2D::_get_total_gravity() const {
	LOG_FUNCTION_CALL("_get_total_gravity")
	ERR_FAIL_COND_V(body->get_space(), Vector2());
	if (body->using_area_gravity) {
		return body->total_gravity;
	} else {
		Vector2 default_gravity = body->get_space()->get_default_area_param(PhysicsServer2D::AREA_PARAM_GRAVITY);
		return body->gravity_scale * default_gravity;
	}
}

double RapierDirectBodyState2D::_get_total_angular_damp() const {
	LOG_FUNCTION_CALL("_get_total_angular_damp")
	ERR_FAIL_COND_V(body->get_space(), 0.0);
	return body->total_angular_damping;
}

double RapierDirectBodyState2D::_get_total_linear_damp() const {
	LOG_FUNCTION_CALL("_get_total_linear_damp")
	ERR_FAIL_COND_V(body->get_space(), 0.0);
	return body->total_linear_damping;
}

Vector2 RapierDirectBodyState2D::_get_center_of_mass() const {
	LOG_FUNCTION_CALL("_get_center_of_mass")
	return body->get_transform().basis_xform(body->get_center_of_mass());
}

Vector2 RapierDirectBodyState2D::_get_center_of_mass_local() const {
	LOG_FUNCTION_CALL("_get_center_of_mass_local")
	return body->get_center_of_mass();
}

double RapierDirectBodyState2D::_get_inverse_mass() const {
	LOG_FUNCTION_CALL("_get_inverse_mass")
	return body->get_inv_mass();
}

double RapierDirectBodyState2D::_get_inverse_inertia() const {
	LOG_FUNCTION_CALL("_get_inverse_inertia")
	return body->get_inv_inertia();
}

void RapierDirectBodyState2D::_set_linear_velocity(const Vector2 &p_velocity) {
	LOG_FUNCTION_CALL("_set_linear_velocity")
	body->set_linear_velocity(p_velocity);
}

Vector2 RapierDirectBodyState2D::_get_linear_velocity() const {
	LOG_FUNCTION_CALL("_get_linear_velocity")
	return body->get_linear_velocity();
}

void RapierDirectBodyState2D::_set_angular_velocity(double p_velocity) {
	LOG_FUNCTION_CALL("_set_angular_velocity")
	body->set_angular_velocity(p_velocity);
}

double RapierDirectBodyState2D::_get_angular_velocity() const {
	LOG_FUNCTION_CALL("_get_angular_velocity")
	return body->get_angular_velocity();
}

void RapierDirectBodyState2D::_set_transform(const Transform2D &p_transform) {
	LOG_FUNCTION_CALL("_set_transform")
	body->set_transform(p_transform, true);
}

Transform2D RapierDirectBodyState2D::_get_transform() const {
	LOG_FUNCTION_CALL("_get_transform")
	return body->get_transform();
}

Vector2 RapierDirectBodyState2D::_get_velocity_at_local_position(const Vector2 &p_position) const {
	LOG_FUNCTION_CALL("_get_velocity_at_local_position")
	return body->get_velocity_at_local_point(p_position);
}

void RapierDirectBodyState2D::_apply_central_impulse(const Vector2 &p_impulse) {
	LOG_FUNCTION_CALL("_apply_central_impulse")
	body->apply_central_impulse(p_impulse);
}

void RapierDirectBodyState2D::_apply_impulse(const Vector2 &p_impulse, const Vector2 &p_position) {
	LOG_FUNCTION_CALL("_apply_impulse")
	body->apply_impulse(p_impulse, p_position);
}

void RapierDirectBodyState2D::_apply_torque_impulse(double p_torque) {
	LOG_FUNCTION_CALL("_apply_torque_impulse")
	body->apply_torque_impulse(p_torque);
}

void RapierDirectBodyState2D::_apply_central_force(const Vector2 &p_force) {
	LOG_FUNCTION_CALL("_apply_central_force")
	body->apply_central_force(p_force);
}

void RapierDirectBodyState2D::_apply_force(const Vector2 &p_force, const Vector2 &p_position) {
	LOG_FUNCTION_CALL("_apply_force")
	body->apply_force(p_force, p_position);
}

void RapierDirectBodyState2D::_apply_torque(double p_torque) {
	LOG_FUNCTION_CALL("_apply_torque")
	body->apply_torque(p_torque);
}

void RapierDirectBodyState2D::_add_constant_central_force(const Vector2 &p_force) {
	LOG_FUNCTION_CALL("_add_constant_central_force")
	body->add_constant_central_force(p_force);
}

void RapierDirectBodyState2D::_add_constant_force(const Vector2 &p_force, const Vector2 &p_position) {
	LOG_FUNCTION_CALL("_add_constant_force")
	body->add_constant_force(p_force, p_position);
}

void RapierDirectBodyState2D::_add_constant_torque(double p_torque) {
	LOG_FUNCTION_CALL("_add_constant_torque")
	body->add_constant_torque(p_torque);
}

void RapierDirectBodyState2D::_set_constant_force(const Vector2 &p_force) {
	LOG_FUNCTION_CALL("_set_constant_force")
	body->set_constant_force(p_force);
}

Vector2 RapierDirectBodyState2D::_get_constant_force() const {
	LOG_FUNCTION_CALL("_get_constant_force")
	return body->get_constant_force();
}

void RapierDirectBodyState2D::_set_constant_torque(double p_torque) {
	LOG_FUNCTION_CALL("_set_constant_torque")
	body->set_constant_torque(p_torque);
}

double RapierDirectBodyState2D::_get_constant_torque() const {
	LOG_FUNCTION_CALL("_get_constant_torque")
	return body->get_constant_torque();
}

void RapierDirectBodyState2D::_set_sleep_state(bool p_enable) {
	LOG_FUNCTION_CALL("_set_sleep_state")
	body->set_active(!p_enable);
}

bool RapierDirectBodyState2D::_is_sleeping() const {
	LOG_FUNCTION_CALL("_is_sleeping")
	return !body->is_active();
}

int RapierDirectBodyState2D::_get_contact_count() const {
	LOG_FUNCTION_CALL("_get_contact_count")
	return body->contact_count;
}

Vector2 RapierDirectBodyState2D::_get_contact_local_position(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_local_position")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, Vector2());
	return body->contacts[p_contact_idx].local_pos;
}

Vector2 RapierDirectBodyState2D::_get_contact_local_normal(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_local_normal")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, Vector2());
	return body->contacts[p_contact_idx].local_normal;
}

int RapierDirectBodyState2D::_get_contact_local_shape(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_local_shape")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, -1);
	return body->contacts[p_contact_idx].local_shape;
}

Vector2 RapierDirectBodyState2D::_get_contact_local_velocity_at_position(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_local_velocity_at_position")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, Vector2());
	return body->contacts[p_contact_idx].local_velocity_at_pos;
}

RID RapierDirectBodyState2D::_get_contact_collider(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_collider")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, RID());
	return body->contacts[p_contact_idx].collider;
}
Vector2 RapierDirectBodyState2D::_get_contact_collider_position(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_collider_position")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, Vector2());
	return body->contacts[p_contact_idx].collider_pos;
}

uint64_t RapierDirectBodyState2D::_get_contact_collider_id(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_collider_id")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, 0);
	return body->contacts[p_contact_idx].collider_instance_id;
}

Object *RapierDirectBodyState2D::_get_contact_collider_object(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_collider_object")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, nullptr);
	return body->contacts[p_contact_idx].collider_object;
}

int RapierDirectBodyState2D::_get_contact_collider_shape(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_collider_shape")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, 0);
	return body->contacts[p_contact_idx].collider_shape;
}

Vector2 RapierDirectBodyState2D::_get_contact_impulse(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_impulse")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, Vector2());
	return body->contacts[p_contact_idx].impulse;
}

Vector2 RapierDirectBodyState2D::_get_contact_collider_velocity_at_position(int p_contact_idx) const {
	LOG_FUNCTION_CALL("_get_contact_collider_velocity_at_position")
	ERR_FAIL_INDEX_V(p_contact_idx, body->contact_count, Vector2());
	return body->contacts[p_contact_idx].collider_velocity_at_pos;
}

PhysicsDirectSpaceState2D *RapierDirectBodyState2D::_get_space_state() {
	LOG_FUNCTION_CALL("_get_space_state")
	return body->get_space()->get_direct_state();
}

double RapierDirectBodyState2D::_get_step() const {
	LOG_FUNCTION_CALL("_get_step")
	return body->get_space()->get_last_step();
}
