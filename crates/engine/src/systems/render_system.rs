#[macro_export]
macro_rules! render_system {
    () => {
        |services: &mut Services, state: &mut AppState, world: &mut World| {
            let renderer = services.resolve::<WebGlRenderer>().unwrap();
            renderer.clear(0.152, 0.214, 0.3, 1.0);
            
            let mut scene_data = None;
            for e in world.query().scenedata().fetch() {
                scene_data = world.get_scenedata(e);
                break;
            }

            if scene_data.is_none() {
                return;
            }

            let scene_data = scene_data.unwrap();

            for e in world.query().camera().transform().fetch() {
                let camera = world.get_camera(e).unwrap();
                let camera_transform = world.get_transform(e).unwrap();

                let (w, h) = state.resolution();
                let projection = math::Matrix4x4::new_perspective(
                    w as f32 / h as f32,
                    camera.fov().to_radians(),
                    camera.near(),
                    camera.far(),
                );
                let view = camera.get_view_matrix();

                for e1 in world.query().drawpackage().fetch() {
                    let batch_list = world.get_drawpackage(e1).unwrap();

                    for (material, frames) in batch_list.meshes.iter() {
                        renderer.update_state(&material.tlu.params());

                        let program = material.program();

                        renderer.use_program(Some(program));

                        renderer.set_uniform_int(material.program(), "material.diffuse", 0);
                        renderer.use_texture(0, material.tex_diffuse());

                        renderer.set_uniform_int(material.program(), "material.specular", 1);
                        renderer.use_texture(1, material.tex_specular());

                        renderer.set_uniform_int(material.program(), "env", 2);
                        renderer.use_cube_texture(2, scene_data.get_env_tetxure());

                        renderer.set_uniform_float(program, "material.shininess", material.shininess);

                        renderer.set_uniform_matrix4(
                            material.program(),
                            "projection",
                            projection.as_slice(),
                        );

                        let (camera_pos_lo, camera_pos_hi) = math::split_double(camera_transform.position());

                        renderer.set_uniform_vector3(program, "view_pos_lo", camera_pos_lo.as_slice());
                        renderer.set_uniform_vector3(program, "view_pos_hi", camera_pos_hi.as_slice());

                        renderer.set_uniform_matrix4(material.program(), "view", view.as_slice());

                        // -- setup lighting

                        renderer.set_uniform_vector3(
                            program,
                            "light_ambient",
                            scene_data.ambient.as_slice(),
                        );

                        for e in world.query().directionallight().transform().fetch() {
                            let light = world.get_directionallight(e).unwrap();
                            let transform = world.get_transform(e).unwrap();

                            renderer.set_uniform_vector3(
                                program,
                                "dirLight.direction",
                                math::to_single_vector(transform.front()).as_slice()
                            );
                            renderer.set_uniform_vector3(
                                program,
                                "dirLight.diffuse",
                                light.diffuse.as_slice(),
                            );
                            renderer.set_uniform_vector3(
                                program,
                                "dirLight.specular",
                                light.specular.as_slice(),
                            );
                        }

                        let mut point_light_index = 0;
                        for e in world.query().pointlight().transform().fetch() {
                            let light = world.get_pointlight(e).unwrap();
                            let transform = world.get_transform(e).unwrap();

                            let (pos_lo, pos_hi) = math::split_double(transform.position());

                            renderer.set_uniform_vector3(
                                program,
                                format!("pointLights[{}].position_low", point_light_index).as_str(),
                                pos_lo.as_slice(),
                            );

                            renderer.set_uniform_vector3(
                                program,
                                format!("pointLights[{}].position_high", point_light_index).as_str(),
                                pos_hi.as_slice(),
                            );
                            renderer.set_uniform_vector3(
                                program,
                                format!("pointLights[{}].diffuse", point_light_index).as_str(),
                                light.diffuse.as_slice(),
                            );
                            renderer.set_uniform_vector3(
                                program,
                                format!("pointLights[{}].specular", point_light_index).as_str(),
                                light.specular.as_slice(),
                            );

                            renderer.set_uniform_float(program, format!("pointLights[{}].constant", point_light_index).as_str(), light.constant);
                            renderer.set_uniform_float(program, format!("pointLights[{}].linear", point_light_index).as_str(), light.linear);
                            renderer.set_uniform_float(program, format!("pointLights[{}].quadratic", point_light_index).as_str(), light.quadratic);

                            point_light_index += 1;
                        }

                        renderer.set_uniform_int(program, "point_light_count", point_light_index);

                        let mut spotlight_index = 0;
                        for e in world.query().spotlight().transform().fetch() {
                            let light = world.get_spotlight(e).unwrap();
                            let transform = world.get_transform(e).unwrap();

                            let (pos_lo, pos_hi) = math::split_double(transform.position());

                            renderer.set_uniform_vector3(
                                program,
                                format!("spotLights[{}].position_low", spotlight_index).as_str(),
                                pos_lo.as_slice(),
                            );

                            renderer.set_uniform_vector3(
                                program,
                                format!("spotLights[{}].position_high", spotlight_index).as_str(),
                                pos_hi.as_slice(),
                            );

                            renderer.set_uniform_vector3(
                                program,
                                format!("spotLights[{}].direction", spotlight_index).as_str(),
                                math::to_single_vector(transform.front()).as_slice(),
                            );

                            renderer.set_uniform_vector3(
                                program,
                                format!("spotLights[{}].diffuse", spotlight_index).as_str(),
                                light.diffuse.as_slice(),
                            );

                            renderer.set_uniform_vector3(
                                program,
                                format!("spotLights[{}].specular", spotlight_index).as_str(),
                                light.specular.as_slice(),
                            );

                            renderer.set_uniform_float(program, format!("spotLights[{}].constant", spotlight_index).as_str(), light.constant);
                            renderer.set_uniform_float(program, format!("spotLights[{}].linear", spotlight_index).as_str(), light.linear);
                            renderer.set_uniform_float(program, format!("spotLights[{}].quadratic", spotlight_index).as_str(), light.quadratic);
                            renderer.set_uniform_float(program, format!("spotLights[{}].cutOff", spotlight_index).as_str(), light.cut_off.to_radians().cos());
                            renderer.set_uniform_float(program, format!("spotLights[{}].outerCutOff", spotlight_index).as_str(), light.outer_cut_off.to_radians().cos());

                            spotlight_index += 1;
                        }

                        renderer.set_uniform_int(program, "spot_light_count", spotlight_index);

                        for frame in frames {
                            renderer.set_uniform_vector3(program, "model_pos_lo", frame.position1.as_slice());
                            renderer.set_uniform_vector3(program, "model_pos_hi", frame.position2.as_slice());

                            renderer.set_uniform_matrix4(
                                program,
                                "model",
                                frame.transform.as_slice(),
                            );

                            renderer.use_mesh(Some(&frame.mesh_id));
                            renderer.draw(frame.index_count);
                        }
                    }
                }

                for e1 in world.query().skybox().fetch() {
                    let skybox = world.get_skybox(e1).unwrap();

                    renderer.update_state(&skybox.shader.params());

                    renderer.use_program(Some(&skybox.program));

                    renderer.set_uniform_matrix4(
                        &skybox.program,
                        "projection",
                        projection.as_slice(),
                    );
                    
                    renderer.set_uniform_matrix4(&skybox.program, "view", view.as_slice());

                    renderer.use_cube_texture(0, Some(&skybox.texture));
                    renderer.set_uniform_int(&skybox.program, "skybox", 0);
                    renderer.use_mesh(Some(&skybox.mesh));
                    renderer.draw_arrays(skybox.vertex_count);
                }
            }
        }
    };
}
