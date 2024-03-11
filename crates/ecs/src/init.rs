#[macro_export]
macro_rules! init_ecs {
    ($($struct_name:ident),*) => {

        use paste::paste;
        use std::collections::HashMap;

        paste::paste! {
            pub struct World {
                $([<$struct_name:lower>]: Vec<$struct_name>,)*
                entities: HashMap<usize,usize>,
                one_frame: Vec<(usize, usize)>,
            }
        }

        impl World {
            paste::paste! {
                pub fn create() -> Self {
                    Self {
                        $([<$struct_name:lower>]: Vec::new(),)*
                        entities: HashMap::new(),
                        one_frame: Vec::new(),
                    }
                }
            }

            pub fn create_entity(&mut self) -> usize {
                for (k, v) in &self.entities {
                    if *v == 0 {
                        return *k
                    }
                }
                let e = self.entities.len();
                self.entities.insert(e, 0);
                e
            }

            fn clear_one_frame(&mut self) {
                for comp in self.one_frame.iter() {
                paste::paste! {
                    $(
                        if $struct_name::get_id() == comp.1 {
                            self.[<$struct_name:lower>].retain(|c| c.entity() != comp.0);
                            if let Some(comp_count) = self.entities.get(&comp.0) {
                                self.entities.insert(comp.0, comp_count - 1);
                            }
                        }
                    )*
                }
            }
                self.one_frame.clear();
            }

            pub fn query(&self) -> Query {
                Query {
                    world: self,
                    query: Vec::new(),
                }
            }

            pub fn stat(&self) -> (usize, usize) {
                let mut comp_count = 0;
                paste::paste! {
                    $(
                        comp_count += self.[<$struct_name:lower>].len();
                    )*
                }
                (self.entities.len(), comp_count)
            }

            // create methods

            $(
                paste::paste! {
                        pub fn [<get_$struct_name:lower>](&self, entity: usize) -> Option<&mut $struct_name> {

                                for item in &self.[<$struct_name:lower>] {
                                    let item = unsafe { &mut *(item as *const _ as *mut $struct_name) };
                                    if item.entity() == entity {
                                        return Some(item)
                                    }
                        }
                            None
                        }
                }
            )*
        }

        // init queries

        pub struct Query<'a> {
            world: &'a World,
            query: Vec<usize>,
        }

        impl<'a> Query<'a> {
            $(
                paste::paste! {
                    pub fn [<$struct_name:lower>](mut self) -> Query<'a> {
                        self.query.push($struct_name::get_id());
                        self
                    }
            }
            )*

            paste::paste! {
                pub fn fetch(&self) -> Vec<usize> {
                    let mut entities: Option<Vec<usize>> = None;
                    for e in self.query.iter() {
                        let mut new_entities = Vec::<usize>::new();
                        match e {
                        $(
                            [<$struct_name:lower>] if *[<$struct_name:lower>] == $struct_name::get_id() => {
                                for comp in &self.world.[<$struct_name:lower>] {
                                    new_entities.push(comp.entity);
                                }
                            },
                        )*
                        _ => (),
                        }
                        if let Some(entities) = &mut entities {
                            entities.retain(|e| new_entities.contains(e));
                        } else {
                            entities = Some(new_entities);
                        }
                    }
                    entities.unwrap()
                }
            }
        }

        // init components

        pub trait AddComponent {
            fn add(self, world: &mut World);
            fn remove(&self, world: &mut World);
        }

        $(
            paste::paste! {
            impl AddComponent for $struct_name {

                    fn add(self, world: &mut World) {
                        if self.one_frame() {
                            world.one_frame.push((self.entity(), $struct_name::get_id()))
                        }
                        world.entities.entry(self.entity()).and_modify(|e| *e += 1);
                        world.[<$struct_name:lower>].push(self);
                    }

                    fn remove(&self, world: &mut World) {
                        let cnt = world.[<$struct_name:lower>].len();
                        world.[<$struct_name:lower>].retain(|c| c.entity() != self.entity());
                        if world.[<$struct_name:lower>].len() != cnt {
                            world.entities.entry(self.entity()).and_modify(|e| *e -= 1);
                        }
                    }
            }
        }
        )*
    };
}
