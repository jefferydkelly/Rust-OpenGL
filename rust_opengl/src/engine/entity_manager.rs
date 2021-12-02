use std::any::Any;
use std::cell::{RefCell, RefMut};

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any_mut(&mut self)-> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;

}

impl <T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None);
    }
}


struct EntityManager {
    entities_count:usize,
    component_vecs:Vec<Box<dyn ComponentVec>>
}

impl EntityManager {
    fn new()-> Self {
       Self {
           entities_count: 0,
           component_vecs: Vec::new()
       }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }

        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity<ComponentType: 'static> (&mut self, entity:usize, component:ComponentType) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec:Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
    }

    fn borrow_component_vec<ComponentType: 'static>(&self)-> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                return Some(component_vec.borrow_mut());
            }
        }
        
        None
    }
}