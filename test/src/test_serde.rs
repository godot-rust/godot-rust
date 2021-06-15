use ::serde::{Deserialize, Serialize};
use gdnative::prelude::*;

pub(crate) fn run_tests() -> bool {
    let mut status = true;
    
    //These [de]serialize Foo via the derived impl, instead of going through ToVariant/FromVariant
    status &= test_ron_round_trip();
    status &= test_json_round_trip();
    status &= test_cbor_round_trip();
    status &= test_yaml_round_trip();
    status &= test_toml_round_trip();
    
    let mut eq_works = true;
    eq_works &= test_variant_eq();
    eq_works &= test_dispatch_eq();
    //All other tests depend on these invariants
    if !eq_works {
        gdnative::godot_error!(
            "   !!!! Can't run remaining serde tests, ToVariant/FromVariant is broken!"
        );
        return false;
    }
    
    status &= test_ron_disp_round_trip();
    status &= test_ron_de_disp_as_variant();
    status &= test_json_disp_round_trip();
    status &= test_json_de_disp_as_variant();
    status &= test_bincode_disp_round_trip();
    status &= test_cbor_variant_round_trip();
    status &= test_cbor_disp_round_trip();
    status &= test_yaml_variant_round_trip();
    status &= test_yaml_disp_round_trip();
    status &= test_toml_variant_round_trip();
    status &= test_toml_disp_round_trip();
    
    status
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToVariant, FromVariant)]
struct Foo {
    some: Option<bool>,
    none: Option<bool>,
    b: bool,
    int: i64,
    float: f64,
    str: GodotString,
    vec2: Vector2,
    // rect2: Rect2, //TODO: PartialEq
    vec3: Vector3,
    // xform_2d: Transform2D, //TODO: PartialEq
    plane: Plane,
    quat: Quat,
    aabb: Aabb,
    basis: Basis,
    xform: Transform,
    color: Color,
    path: NodePath,
    rid: Rid,
    // obj: Object, //TODO: how best to test this?
    // dict: Dictionary, //TODO: PartialEq
    // v_arr: VariantArray, //TODO: PartialEq
    byte_arr: ByteArray,
    int_arr: Int32Array,
    float_arr: Float32Array,
    str_arr: StringArray,
    vec2_arr: Vector2Array,
    vec3_arr: Vector3Array,
    color_arr: ColorArray,
}

impl Foo {
    fn new() -> Self {
        Self {
            some: Some(true),
            none: None,
            b: false,
            int: 1,
            float: 2.0,
            str: "this is a str".into(),
            vec2: Vector2::RIGHT,
            vec3: Vector3::BACK,
            plane: Plane {
                normal: Vector3::ONE.normalized(),
                d: 3.0,
            },
            quat: Quat::new(4.1, 5.2, 6.3, 7.5),
            aabb: Aabb {
                position: Vector3::new(8.2, 9.8, 10.11),
                size: Vector3::new(12.13, 14.15, 16.17),
            },
            basis: Basis::identity().rotated(Vector3::UP, std::f32::consts::TAU / 3.0),
            xform: Transform {
                basis: Basis::from_euler(Vector3::new(18.19, 20.21, 22.23)),
                origin: Vector3::new(24.25, 26.27, 28.29),
            },
            color: Color::from_rgb(0.549, 0.0, 1.0),
            path: "/root/Node".into(),
            rid: Rid::new(),
            byte_arr: ByteArray::from_slice(&[30u8, 31u8, 32u8]),
            int_arr: Int32Array::from_slice(&[33i32, 34i32, 35i32, 36i32]),
            float_arr: Float32Array::from_slice(&[37.38, 39.40]),
            str_arr: StringArray::from_vec(vec!["hello".into(), "world".into()]),
            vec2_arr: Vector2Array::from_slice(&[
                Vector2::UP,
                Vector2::UP,
                Vector2::DOWN,
                Vector2::DOWN,
                Vector2::LEFT,
                Vector2::RIGHT,
                Vector2::LEFT,
                Vector2::RIGHT,
            ]),
            vec3_arr: Vector3Array::from_slice(&[
                Vector3::ONE * 41.0,
                Vector3::BACK * 42.43,
                Vector3::FORWARD * 44.45,
            ]),
            color_arr: ColorArray::from_slice(&[Color::from_rgba(0.0, 1.0, 0.627, 0.8)]),
        }
    }
}

/// Sanity check that a round trip through Variant preserves equality for Foo.
fn test_variant_eq() -> bool {
    println!(" -- test_variant_eq");

    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let variant = foo.to_variant();
        let result = Foo::from_variant(&variant).unwrap();
        assert_eq!(foo, result);
    })
    .is_ok();

    if !ok {
        gdnative::godot_error!("   !! Test test_variant_eq failed");
    }

    ok
}

/// Sanity check that a round trip through VariantDispatch preserves equality for Foo.
fn test_dispatch_eq() -> bool {
    println!(" -- test_variant_eq");

    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let dispatch = foo.to_variant().dispatch();
        let result = Foo::from_variant(&Variant::from(&dispatch)).unwrap();
        assert_eq!(foo, result);
    })
    .is_ok();

    if !ok {
        gdnative::godot_error!("   !! Test test_dispatch_eq failed");
    }

    ok
}

fn test_ron_round_trip() -> bool {
    println!(" -- test_ron_round_trip");

    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let test_str = ron::to_string(&foo);
        let mut de = ron::Deserializer::from_str(test_str.as_ref().unwrap());
        let result = Foo::deserialize(de.as_mut().unwrap()).unwrap();
        assert_eq!(foo, result)
    })
    .is_ok();

    if !ok {
        gdnative::godot_error!("   !! Test test_ron_round_trip failed");
    }

    ok
}

fn test_ron_disp_round_trip() -> bool {
    println!(" -- test_ron_disp_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let test_str = ron::to_string(&foo.to_variant().dispatch());
        let mut de = ron::Deserializer::from_str(test_str.as_ref().unwrap());
        let disp = VariantDispatch::deserialize(de.as_mut().unwrap()).unwrap();
        let result = Foo::from_variant(&Variant::from(&disp)).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_ron_disp_round_trip failed");
    }
    
    ok
}

fn test_ron_de_disp_as_variant() -> bool {
    println!(" -- test_ron_de_disp_as_variant");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = ron::to_string(&foo.to_variant().dispatch());
        let mut de = ron::Deserializer::from_str(s.as_ref().unwrap());
        let variant = Variant::deserialize(de.as_mut().unwrap()).unwrap();
        let result = Foo::from_variant(&variant).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_ron_de_disp_as_variant failed");
    }
    
    ok
}

fn test_json_round_trip() -> bool {
    println!(" -- test_json_round_trip");

    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = serde_json::to_string(&foo);
        let result = serde_json::from_str::<Foo>(s.as_ref().unwrap()).unwrap();
        assert_eq!(foo, result)
    })
    .is_ok();

    if !ok {
        gdnative::godot_error!("   !! Test test_json_round_trip failed");
    }

    ok
}

fn test_json_disp_round_trip() -> bool {
    println!(" -- test_json_disp_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = serde_json::to_string(&foo.to_variant().dispatch());
        let disp = serde_json::from_str::<VariantDispatch>(s.as_ref().unwrap()).unwrap();
        let result = Foo::from_variant(&Variant::from(&disp)).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_json_disp_round_trip failed");
    }
    
    ok
}

fn test_json_de_disp_as_variant() -> bool {
    println!(" -- test_json_de_disp_as_variant");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = serde_json::to_string(&foo.to_variant().dispatch());
        let variant = serde_json::from_str::<Variant>(s.as_ref().unwrap()).unwrap();
        let result = Foo::from_variant(&variant).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_json_de_disp_as_variant failed");
    }
    
    ok
}

fn test_bincode_disp_round_trip() -> bool {
    println!(" -- test_bincode_disp_round_trip");

    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let bytes = bincode::serialize(&foo.to_variant().dispatch());
        let disp = bincode::deserialize::<VariantDispatch>(bytes.as_ref().unwrap()).unwrap();
        let result = Foo::from_variant(&Variant::from(&disp)).unwrap();
        assert_eq!(foo, result)
    })
    .is_ok();

    if !ok {
        gdnative::godot_error!("   !! Test test_bincode_disp_round_trip failed");
    }

    ok
}

fn test_cbor_round_trip() -> bool {
    println!(" -- test_cbor_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let bytes = serde_cbor::to_vec(&foo).unwrap();
        let result = serde_cbor::from_slice::<Foo>(&bytes).unwrap();
        assert_eq!(foo, result)
    })
    .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_cbor_round_trip failed");
    }
    
    ok
}


fn test_cbor_variant_round_trip() -> bool {
    println!(" -- test_cbor_variant_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let bytes = serde_cbor::to_vec(&foo.to_variant()).unwrap();
        let disp = serde_cbor::from_slice::<Variant>(&bytes).unwrap();
        let result = Foo::from_variant(&disp).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_cbor_variant_round_trip failed");
    }
    
    ok
}

fn test_cbor_disp_round_trip() -> bool {
    println!(" -- test_cbor_disp_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let bytes = serde_cbor::to_vec(&foo.to_variant().dispatch()).unwrap();
        let disp = serde_cbor::from_slice::<VariantDispatch>(&bytes).unwrap();
        let result = Foo::from_variant(&Variant::from(&disp)).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_cbor_disp_round_trip failed");
    }
    
    ok
}

fn test_yaml_round_trip() -> bool {
    println!(" -- test_yaml_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = serde_yaml::to_string(&foo).unwrap();
        let result = serde_yaml::from_str::<Foo>(&s).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_yaml_round_trip failed");
    }
    
    ok
}

fn test_yaml_variant_round_trip() -> bool {
    println!(" -- test_yaml_variant_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = serde_yaml::to_string(&foo.to_variant()).unwrap();
        let variant = serde_yaml::from_str::<Variant>(&s).unwrap();
        let result = Foo::from_variant(&variant).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_yaml_variant_round_trip failed");
    }
    
    ok
}

fn test_yaml_disp_round_trip() -> bool {
    println!(" -- test_yaml_disp_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = serde_yaml::to_string(&foo.to_variant().dispatch()).unwrap();
        let disp = serde_yaml::from_str::<VariantDispatch>(&s).unwrap();
        let result= Foo::from_variant(&Variant::from(&disp)).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_yaml_disp_round_trip failed");
    }
    
    ok
}

fn test_toml_round_trip() -> bool {
    println!(" -- test_toml_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = toml::to_string(&foo).unwrap();
        let result = toml::from_str::<Foo>(&s).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_toml_round_trip failed");
    }
    
    ok
}

fn test_toml_variant_round_trip() -> bool {
    println!(" -- test_toml_variant_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = toml::to_string(&foo.to_variant()).unwrap();
        let variant = toml::from_str::<Variant>(&s).unwrap();
        let result = Foo::from_variant(&variant).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_toml_variant_round_trip failed");
    }
    
    ok
}

fn test_toml_disp_round_trip() -> bool {
    println!(" -- test_toml_disp_round_trip");
    
    let ok = std::panic::catch_unwind(|| {
        let foo = Foo::new();
        let s = toml::to_string(&foo.to_variant().dispatch()).unwrap();
        let disp = toml::from_str::<VariantDispatch>(&s).unwrap();
        let result= Foo::from_variant(&Variant::from(&disp)).unwrap();
        assert_eq!(foo, result)
    })
      .is_ok();
    
    if !ok {
        gdnative::godot_error!("   !! Test test_toml_disp_round_trip failed");
    }
    
    ok
}