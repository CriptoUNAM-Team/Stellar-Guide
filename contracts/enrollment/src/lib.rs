#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub course_id: u64,
    pub name: String,
    pub capacity: u32,
    pub enrolled: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextId,
    Course(u64),
    Seat(u64, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EnrollmentError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    CourseNotFound = 3,
    AlreadyEnrolled = 4,
    NotEnrolled = 5,
    CourseFull = 6,
    InvalidCapacity = 7,
}

#[contract]
pub struct EnrollmentContract;

#[contractimpl]
impl EnrollmentContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), EnrollmentError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(EnrollmentError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    pub fn create_course(env: Env, name: String, capacity: u32) -> Result<u64, EnrollmentError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(EnrollmentError::NotInitialized)?;
        admin.require_auth();
        if capacity == 0 {
            return Err(EnrollmentError::InvalidCapacity);
        }
        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .ok_or(EnrollmentError::NotInitialized)?;
        env.storage().persistent().set(
            &DataKey::Course(id),
            &Course {
                course_id: id,
                name,
                capacity,
                enrolled: 0,
            },
        );
        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        Ok(id)
    }

    pub fn enroll(env: Env, course_id: u64, student: Address) -> Result<(), EnrollmentError> {
        student.require_auth();
        let mut course: Course = env
            .storage()
            .persistent()
            .get(&DataKey::Course(course_id))
            .ok_or(EnrollmentError::CourseNotFound)?;
        let seat = DataKey::Seat(course_id, student.clone());
        if env.storage().persistent().has(&seat) {
            return Err(EnrollmentError::AlreadyEnrolled);
        }
        if course.enrolled >= course.capacity {
            return Err(EnrollmentError::CourseFull);
        }
        course.enrolled += 1;
        env.storage()
            .persistent()
            .set(&DataKey::Course(course_id), &course);
        env.storage().persistent().set(&seat, &true);
        Ok(())
    }

    pub fn drop_course(env: Env, course_id: u64, student: Address) -> Result<(), EnrollmentError> {
        student.require_auth();
        let mut course: Course = env
            .storage()
            .persistent()
            .get(&DataKey::Course(course_id))
            .ok_or(EnrollmentError::CourseNotFound)?;
        let seat = DataKey::Seat(course_id, student);
        if !env.storage().persistent().has(&seat) {
            return Err(EnrollmentError::NotEnrolled);
        }
        env.storage().persistent().remove(&seat);
        course.enrolled -= 1;
        env.storage()
            .persistent()
            .set(&DataKey::Course(course_id), &course);
        Ok(())
    }

    pub fn get_course(env: Env, course_id: u64) -> Result<Course, EnrollmentError> {
        env.storage()
            .persistent()
            .get(&DataKey::Course(course_id))
            .ok_or(EnrollmentError::CourseNotFound)
    }

    pub fn is_enrolled(env: Env, course_id: u64, student: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Seat(course_id, student))
    }
}

mod test;
