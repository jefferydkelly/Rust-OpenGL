use once_cell::sync::OnceCell;

static mut OBJECT_POOL:OnceCell<ObjectPool> = OnceCell::new();

pub struct ObjectPool {
    
}

impl ObjectPool {
    pub fn create_instance() {
        let op = ObjectPool{};
        unsafe {
            OBJECT_POOL.set(op);
        }
    }

    pub fn get_instance()->&'static mut ObjectPool {
        unsafe {
            return OBJECT_POOL.get_mut().expect("The Object Pool doesn't exist");
        }
    }
}