use x3d::*;

fn trs(translate: Vec3, rotate: [f32; 3], scale: f32) -> Mat4 {
    Mat4::from_translation(translate) * Mat4::from_scale(scale) * Mat4::from_angle_x(Deg(rotate[0]))
        * Mat4::from_angle_y(Deg(rotate[1])) * Mat4::from_angle_z(Deg(rotate[2]))
}

pub fn make_scene() -> Scene {
    let mut scene = Scene::new();

    let white = Color::from_rgb(1.0, 1.0, 1.0);

    // 球1
    let mut s1 = Entity::new(
        trs(vec3(0.0, 0.2, 0.0), [0.0, 0.0, 0.0], 1.0),
        Box::new(Sphere::new(0.5)),
    );
    s1.material.albedo = white;
    //s1.material.emission = white * 4.0;
    //s1.material.reflect = 0.8;
    scene.objects.push(Box::new(s1));

    // 球2
    let mut s2 = Entity::new(
        Mat4::from_translation(vec3(0.2, 0.3, 0.5)),
        Box::new(Sphere::new(0.5)),
    );
    s2.material.albedo = white;
    //scene.objects.push(Box::new(s2));

    // ライト
    let mut light = Entity::new(
        trs(vec3(0.0, -0.799 * 1.0, 0.0), [-90.0, 0.0, 0.0], 0.7),
        Box::new(Rect::new()),
    );
    light.material.emission = white * 8.0;
    scene.objects.push(Box::new(light));

    // 天井
    let mut roof = Entity::new(
        trs(vec3(0.0, -0.801 * 1.0, 0.0), [90.0, 0.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    roof.material.albedo = white;
    scene.objects.push(Box::new(roof));

    // 床
    let mut floor = Entity::new(
        trs(vec3(0.0, 0.801, 0.0), [-90.0, 0.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    floor.material.albedo = white;
    scene.objects.push(Box::new(floor));

    // 後ろの壁
    let mut w_back = Entity::new(
        trs(vec3(0.0, 0.0, 0.803), [0.0, 0.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_back.material.albedo = white;
    scene.objects.push(Box::new(w_back));

    // 手前の壁
    let mut w_front = Entity::new(
        trs(vec3(0.0, 0.0, -0.803), [0.0, 180.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_front.material.albedo = white;
    scene.objects.push(Box::new(w_front));

    // 右の壁
    let mut w_right = Entity::new(
        trs(vec3(0.8, 0.0, 0.0), [0.0, 90.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_right.material.albedo = Color::from_rgb(2.0, 0.2, 0.1);
    w_right.material.emission = Color::from_rgb(0.5, 0.0, 0.0);
    scene.objects.push(Box::new(w_right));

    // 左の壁
    let mut w_left = Entity::new(
        trs(vec3(-0.8, 0.0, 0.0), [0.0, -90.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_left.material.albedo = Color::from_rgb(0.2, 0.8, 0.1);
    scene.objects.push(Box::new(w_left));

    scene
}
