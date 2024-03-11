use ecs::init_ecs;

use ecs::Component;

use component::component;

#[component]
struct Person {
    name: String,
}

#[component]
struct Greeter {
    greeting: String,
}

init_ecs!{Person, Greeter}

#[test]
fn test_query() {
    let world = &mut World::create();
    let e = world.create_entity();
    Person {
        entity: e,
        one_frame: false,
        name: "Tom".to_string(),
    }.add(world);

    assert_eq!(0, world.query().greeter().person().fetch().len());

    Greeter {
        entity: e,
        one_frame: true,
        greeting: "Hello".to_string(),
    }.add(world);

    assert_eq!(1, world.query().person().greeter().fetch().len());
}