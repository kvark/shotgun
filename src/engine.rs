const SURFACE_HEIGHT: f32 = 100.0;

pub enum Kind {
    Player,
    Bullet,
    Enemy,
}

struct State {
    kind: Kind,
    velocity: glam::Vec3,
    acceleration: glam::Vec3,
    stay_on_screen: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    node: baryon::NodeRef,
    entity: baryon::EntityRef,
}

pub struct RichObject<'a> {
    entity: hecs::EntityRef<'a>,
    node: &'a mut baryon::Node,
    node_ref: baryon::NodeRef,
}

impl RichObject<'_> {
    pub fn stay_on_screen(self, stay: bool) -> Self {
        self.entity.get_mut::<State>().unwrap().stay_on_screen = stay;
        self
    }

    pub fn position(self, x: f32, y: f32) -> Self {
        self.node.set_position([x, y, 0.0].into());
        self
    }

    pub fn velocity(self, x: f32, y: f32) -> Self {
        self.entity.get_mut::<State>().unwrap().velocity = glam::Vec3::new(x, y, 0.0);
        self
    }

    pub fn acceleration(self, x: f32, y: f32) -> Self {
        self.entity.get_mut::<State>().unwrap().acceleration = glam::Vec3::new(x, y, 0.0);
        self
    }

    pub fn finish(self) -> Object {
        Object {
            node: self.node_ref,
            entity: self.entity.entity(),
        }
    }
}

pub struct Engine {
    pub context: baryon::Context,
    scene: baryon::Scene,
    camera: baryon::Camera,
    pass: baryon::pass::Flat,
}

impl Engine {
    pub fn new(window: &baryon::window::Window) -> Self {
        let context = pollster::block_on(baryon::Context::init().build(window));
        let aspect = context.surface_info().unwrap().aspect_ratio;
        let scene = baryon::Scene::new();
        let extent = 0.5 * SURFACE_HEIGHT;
        let camera = baryon::Camera {
            projection: baryon::Projection::Orthographic {
                center: [aspect * extent, extent].into(),
                extent_y: extent,
            },
            ..Default::default()
        };
        let pass = baryon::pass::Flat::new(&context);
        Self {
            context,
            scene,
            camera,
            pass,
        }
    }

    pub fn screen_size(&self) -> (f32, f32) {
        let aspect = self.context.surface_info().unwrap().aspect_ratio;
        (aspect * SURFACE_HEIGHT, SURFACE_HEIGHT)
    }

    pub fn spawn(
        &mut self,
        kind: Kind,
        image: baryon::ImageRef,
        uv_rect: baryon::UvRange,
    ) -> RichObject {
        let node = self.scene.add_node().build();
        let state = State {
            kind,
            stay_on_screen: false,
            velocity: Default::default(),
            acceleration: Default::default(),
        };
        let entity = self
            .scene
            .add_sprite(image)
            .uv(uv_rect)
            .parent(node)
            .component(state)
            .build();

        RichObject {
            entity: self.scene.world.entity(entity).unwrap(),
            node: &mut self.scene.nodes[node],
            node_ref: node,
        }
    }

    pub fn with(&mut self, object: Object) -> RichObject {
        RichObject {
            entity: self.scene.world.entity(object.entity).unwrap(),
            node: &mut self.scene.nodes[object.node],
            node_ref: object.node,
        }
    }

    pub fn draw(&mut self) {
        self.context
            .present(&mut self.pass, &self.scene, &self.camera);
    }
}
