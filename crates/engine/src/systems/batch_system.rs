#[macro_export]
macro_rules! batch_system {
    () => {
        |services: &mut Services, state: &mut AppState, world: &mut World| {
            let entity_batch_list = world.create_entity();
            let mut batch_list = DrawPackage {
                entity: entity_batch_list,
                one_frame: true,
                meshes: HashMap::new(),
            };

            for e in world.query().mesh().transform().fetch() {
                let mesh = world.get_mesh(e).unwrap();
                let transform = world.get_transform(e).unwrap();

                if let Some(mat) = &mesh.material {
                    if !batch_list.meshes.contains_key(mat) {
                        batch_list.meshes.insert(Rc::clone(mat), Vec::new());
                    }
                    if let Some(batch) = batch_list.meshes.get_mut(mat) {

                        let (position1, position2) = math::split_double(transform.position());

                        batch.push(DrawCallInfo {
                            mesh_id: Rc::clone(&mesh.mesh_id),
                            transform: transform.get_matrix(),
                            index_count: mesh.mesh_data.raw_indices().len() as i32,
                            position1,
                            position2,
                        });
                    }
                }
            }

            batch_list.add(world);
        }
    };
}
