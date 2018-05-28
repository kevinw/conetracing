extern crate unrust;
extern crate num;

#[macro_use]
extern crate unrust_derive;

use unrust::world::{Actor, World, WorldBuilder, /* Handle */};
use unrust::engine::{Camera, DirectionalLight, GameObject, Material, Mesh,
//                     RenderTexture, TextureAttachment, ClearOption
};
use unrust::world::events::*;
use unrust::math::*;

// GUI
use unrust::imgui;

//use std::rc::Rc;

#[derive(Actor)]
pub struct MainScene {
    eye: Vector3<f32>,
    last_event: Option<AppEvent>,
}


fn spring(x:&mut f32, v:&mut f32, xt: f32, zeta: f32, omega: f32, delta_time: f32) -> () {
    let f:f32 = 1.0f32 + 2.0f32 * delta_time * zeta * omega;
    let oo:f32 = omega * omega;
    let hoo:f32 = delta_time * oo;
    let hhoo:f32 = delta_time * hoo;
    let det_inv:f32 = 1.0f32 / (f + hhoo);
    let det_x:f32 = f * *x + delta_time * *v + hhoo * xt;
    let det_v:f32 = *v + hoo * (xt - *x);
    *x = det_x * det_inv;
    *v = det_v * det_inv;
}


impl MainScene {
    fn new() -> MainScene {
        MainScene {
            eye: Vector3::new(-3.0, 3.0, -3.0),
            last_event: None,
        }
    }
}

impl Actor for MainScene {
    fn start(&mut self, _go: &mut GameObject, world: &mut World) {
        // add main camera to scene
        {
            let go = world.new_game_object();
            go.borrow_mut().add_component(Camera::default());
        }

        // add direction light to scene.
        let go = world.new_game_object();
        go.borrow_mut()
            .add_component(DirectionalLight::default());

        // Added Crt
        let go = world.new_game_object();
        go.borrow_mut().add_component(Crt::new());
    }

    fn update(&mut self, _go: &mut GameObject, world: &mut World) {
        // Handle Events
        {
            let target = Vector3::new(0.0, 0.0, 0.0);
            let front = (self.eye - target).normalize();
            let mut reset = false;

            for evt in world.events().iter() {
                self.last_event = Some(evt.clone());
                match evt {
                    &AppEvent::KeyDown(ref key) => {
                        match key.code.as_str() {
                            "KeyA" => self.eye = Quaternion::from_angle_y(Rad(-0.2)) * self.eye,
                            "KeyD" => self.eye = Quaternion::from_angle_y(Rad(0.2)) * self.eye,
                            "KeyW" => self.eye -= front * 2.0,
                            "KeyS" => self.eye += front * 2.0,
                            "Escape" => reset = true,
                            _ => (),
                        };
                    }

                    _ => (),
                }
            }

            if reset {
                world.reset();
                // Because reset will remove all objects in the world,
                // included this Actor itself
                // so will need to add it back.
                let scene = world.new_game_object();
                scene.borrow_mut().add_component(MainScene::new());
                return;
            }
        }

        // Update Camera
        {
            let cam = world.current_camera().unwrap();

            cam.borrow_mut().lookat(
                &Point3::from_vec(self.eye),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 1.0, 0.0),
            );
        }

        // GUI
        use imgui::Metric::*;

        imgui::pivot((1.0, 1.0));
        imgui::label(
            Native(1.0, 1.0) - Pixel(8.0, 8.0),
            "[WASD] : control camera\n[Esc]  : reload all (include assets)",
        );

        imgui::pivot((1.0, 0.0));
        imgui::label(
            Native(1.0, 0.0) + Pixel(-8.0, 8.0),
            &format!("last event: {:?}", self.last_event),
        );
    }
}


struct Spring {
    pub pos: Vector3f,
    pub vel: Vector3f,
    pub target: Vector3f,

    pub zeta: f32,
    pub omega: f32,
}


impl Spring where 
{
    fn new(pos: Vector3f, vel: Vector3f, target: Vector3f, zeta: f32, omega: f32) -> Spring {
        Spring { pos, vel, target, zeta, omega }
    }
    
    fn update(&mut self, delta_time: f32) {
        let f:f32 = 1.0_f32 + 2.0_f32 * delta_time * self.zeta * self.omega;
        let oo = self.omega * self.omega;
        let hoo = delta_time * oo;
        let hhoo = delta_time * hoo;
        let det_inv = 1.0f32 / (f + hhoo);
        let det_pos = self.pos * f + delta_time * self.vel + self.target * hhoo;
        let det_vel = self.vel + hoo * (self.target - self.pos);
        self.pos = det_pos * det_inv;
        self.vel = det_vel * det_inv;
    }
}

#[derive(Actor)]
pub struct Crt {
    //rt: Rc<RenderTexture>,
    //cube: Handle<GameObject>,
    time: f64,
    frame_count: i32,

    spring: Spring,
}


impl Crt {
    fn new() -> Crt {
        Crt {
            //rt: Rc::new(RenderTexture::new(1024, 1024, TextureAttachment::Color0)),
            //cube: GameObject::empty(),
            time: 0f64,
            frame_count: 0,

            spring: Spring::new(Vector3f::zero(), Vector3f::zero(), Vector3f::zero(), 0.15f32, 8.0f32 * std::f32::consts::PI),

        }
    }
}

impl Actor for Crt {
    fn start(&mut self, go: &mut GameObject, world: &mut World) {
        // add main camera to scene
        {
            let camera_go = world.new_game_object();
            camera_go.borrow_mut().add_component(Camera::default());
        }

        {
            let db = &mut world.asset_system();

            let material = Material::new(db.new_program("conetracing"));
            //material.set("uDiffuse", self.rt.as_texture());
            material.set("iResolution", Vector3::new(800f32, 600f32, 1f32));

            let mut mesh = Mesh::new();
            mesh.add_surface(db.new_mesh_buffer("screen_quad"), material);
            go.add_component(mesh);
        }
    }

    fn update(&mut self, go: &mut GameObject, world: &mut World) {
        for evt in world.events().iter() {
            match evt {
                &AppEvent::MousePos((x, y)) => {
                    self.spring.vel = Vector3f::zero();
                    self.spring.target = Vector3f::new(x as f32, y as f32, 0f32);
                },
                _ => ()
            }
        }

        // Setup fb for camera
        //let cam_borrow = world.current_camera().unwrap();
        //let mut cam = cam_borrow.borrow_mut();

        //cam.render_texture = Some(self.rt.clone());

        // Setup proper viewport to render to the whole texture
        //cam.rect = Some(((0, 0), (1024, 1024)));

        {
            let result = go.find_component::<Mesh>();
            if let Some((ref mesh, _)) = result {
                let material = &mesh.surfaces[0].material;
                let dt = world.delta_time();
                self.time += dt;
                material.set("iTime", self.time as f32);
                material.set("iTimeDelta", dt as f32);
                material.set("iFrame", self.frame_count);
                let zeta = 0.15f32;
                let omega = 8.0f32 * std::f32::consts::PI;
                self.spring.update(dt as f32);
                material.set("spring_pos", self.spring.pos);
            }
        }

        /*
        // show only cube
        // TODO it is a little bit hacky, we should support a PostProcessing Component
        self.cube.borrow_mut().active = true;
        go.active = false;

        // Render current scene by camera using given frame buffer
        world.engine_mut().render_pass(&cam, ClearOption::default());

        // show only this crt
        self.cube.borrow_mut().active = false;
        go.active = true;

        // Clean up stuffs in camera, as later we could render normally
        cam.render_texture = None;
        cam.rect = None;
        */
    }
}

/*
#[derive(Actor)]
pub struct Cube {}

impl Cube {
    fn new() -> Cube {
        Cube {}
    }
}

impl Actor for Cube {
    fn start(&mut self, go: &mut GameObject, world: &mut World) {
        let db = &mut world.asset_system();

        let material = Material::new(db.new_program("phong"));
        material.set("uMaterial.diffuse", db.new_texture("tex_a.png"));
        material.set("uMaterial.shininess", 32.0);

        let mut mesh = Mesh::new();
        mesh.add_surface(db.new_mesh_buffer("cube"), material);
        go.add_component(mesh);
    }

    fn update(&mut self, go: &mut GameObject, _world: &mut World) {
        let mut gtran = go.transform.global();
        let axis = Vector3::new(0.01, 0.02, 0.005);
        let len = axis.magnitude();

        gtran.rot = gtran.rot * Quaternion::from_axis_angle(axis.normalize(), Rad(len));
        go.transform.set_global(gtran);
    }
}
*/

pub fn main() {
    let mut world = WorldBuilder::new("conetracing")
        .with_size((800, 600))
        .with_stats(true)
        .build();

    // Add the main scene as component of scene game object
    let scene = world.new_game_object();
    scene.borrow_mut().add_component(MainScene::new());
    drop(scene);

    world.event_loop();
}
