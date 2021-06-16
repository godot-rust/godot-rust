// use super::super::{
//     dictionary::serde::DictionaryVisitor, variant_array::serde::VariantArrayVisitor,
// };
use super::*;
use ::serde::{
    de::{EnumAccess, Error, MapAccess, SeqAccess, VariantAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};
use once_cell::sync::Lazy;
use std::fmt::Formatter;

// impl Serialize for Variant {
//     #[inline]
//     fn serialize<S>(&self, ser: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
//     where
//         S: Serializer,
//     {
//         match self.dispatch() {
//             VariantDispatch::Nil => ser.serialize_none(),
//             VariantDispatch::Bool(v) => ser.serialize_bool(v),
//             VariantDispatch::I64(v) => ser.serialize_i64(v),
//             VariantDispatch::F64(v) => ser.serialize_f64(v),
//             VariantDispatch::GodotString(v) => ser.serialize_str(&v.to_string()),
//             VariantDispatch::Vector2(v) => v.serialize(ser),
//             VariantDispatch::Rect2(v) => v.serialize(ser),
//             VariantDispatch::Vector3(v) => v.serialize(ser),
//             VariantDispatch::Transform2D(v) => v.serialize(ser),
//             VariantDispatch::Plane(v) => v.serialize(ser),
//             VariantDispatch::Quat(v) => v.serialize(ser),
//             VariantDispatch::Aabb(v) => v.serialize(ser),
//             VariantDispatch::Basis(v) => v.serialize(ser),
//             VariantDispatch::Transform(v) => v.serialize(ser),
//             VariantDispatch::Color(v) => v.serialize(ser),
//             VariantDispatch::NodePath(v) => v.serialize(ser),
//             VariantDispatch::Rid(_rid) => ser.serialize_newtype_variant(
//                 "Variant",
//                 VariantType::Rid as u32,
//                 VariantType::Rid.name(),
//                 &(),
//             ),
//             VariantDispatch::Object(_object) => ser.serialize_newtype_variant(
//                 "Variant",
//                 VariantType::Object as u32,
//                 VariantType::Object.name(),
//                 &(),
//             ),
//             VariantDispatch::Dictionary(v) => v.serialize(ser),
//             VariantDispatch::VariantArray(v) => v.serialize(ser),
//             VariantDispatch::ByteArray(v) => v.serialize(ser),
//             VariantDispatch::Int32Array(v) => v.serialize(ser),
//             VariantDispatch::Float32Array(v) => v.serialize(ser),
//             VariantDispatch::StringArray(v) => v.serialize(ser),
//             VariantDispatch::Vector2Array(v) => v.serialize(ser),
//             VariantDispatch::Vector3Array(v) => v.serialize(ser),
//             VariantDispatch::ColorArray(v) => v.serialize(ser),
//         }
//     }
// }

/// This allows (de)serializing to/from non-self-describing formats by avoiding serializing `Variant`s
// Can't just use a HashMap because VariantDispatch doesn't implement Hash, and this avoids cloning all of the entries anyway
struct DictionaryDispatch(Dictionary);

#[derive(Serialize, Deserialize)]
struct DictionaryDispatchEntry {
    key: VariantDispatch,
    value: VariantDispatch,
}

impl<'d> Serialize for DictionaryDispatch {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_seq(Some(self.0.len() as usize))?;
        for (key, value) in self.0.iter() {
            ser.serialize_element(&DictionaryDispatchEntry {
                key: key.dispatch(),
                value: value.dispatch(),
            })?;
        }
        ser.end()
    }
}

impl<'de> Deserialize<'de> for DictionaryDispatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DictionaryDispatchVisitor;
        impl<'de> Visitor<'de> for DictionaryDispatchVisitor {
            type Value = DictionaryDispatch;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of VariantDispatch pairs")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let dict = Dictionary::new();
                while let Some(DictionaryDispatchEntry { key, value }) = seq.next_element()? {
                    dict.insert(Variant::from(&key), Variant::from(&value))
                }
                Ok(DictionaryDispatch(dict.into_shared()))
            }
        }
        deserializer.deserialize_seq(DictionaryDispatchVisitor)
    }
}

impl Serialize for VariantDispatch {
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use VariantDispatch::*;
        macro_rules! newtype_variant {
            ($t:expr, $v:expr) => {
                ser.serialize_newtype_variant("VariantDispatch", $t as u32, $t.name(), $v)
            };
        }
        match self {
            Nil => ser.serialize_unit_variant(
                "VariantDispatch",
                VariantType::Nil as u32,
                VariantType::Nil.name(),
            ),
            Bool(v) => newtype_variant!(VariantType::Bool, v),
            I64(v) => newtype_variant!(VariantType::I64, v),
            F64(v) => newtype_variant!(VariantType::F64, v),
            GodotString(v) => newtype_variant!(VariantType::GodotString, v),
            Vector2(v) => newtype_variant!(VariantType::Vector2, v),
            Rect2(v) => newtype_variant!(VariantType::Rect2, v),
            Vector3(v) => newtype_variant!(VariantType::Vector3, v),
            Transform2D(v) => newtype_variant!(VariantType::Transform2D, v),
            Plane(v) => newtype_variant!(VariantType::Plane, v),
            Quat(v) => newtype_variant!(VariantType::Quat, v),
            Aabb(v) => newtype_variant!(VariantType::Aabb, v),
            Basis(v) => newtype_variant!(VariantType::Basis, v),
            Transform(v) => newtype_variant!(VariantType::Transform, v),
            Color(v) => newtype_variant!(VariantType::Color, v),
            NodePath(v) => newtype_variant!(VariantType::NodePath, v),
            Rid(v) => newtype_variant!(VariantType::Rid, v),
            Object(_) => newtype_variant!(VariantType::Object, &Option::<()>::None),
            Dictionary(v) => {
                newtype_variant!(VariantType::Dictionary, &DictionaryDispatch(v.new_ref()))
            }
            VariantArray(v) => {
                //Allows serializing to non-self-describing formats by avoiding serializing `Variant`s
                let vec = v.iter().map(|v| v.dispatch()).collect::<Vec<_>>();
                newtype_variant!(VariantType::VariantArray, &vec)
            }
            ByteArray(v) => newtype_variant!(VariantType::ByteArray, v),
            Int32Array(v) => newtype_variant!(VariantType::Int32Array, v),
            Float32Array(v) => newtype_variant!(VariantType::Float32Array, v),
            StringArray(v) => newtype_variant!(VariantType::StringArray, v),
            Vector2Array(v) => newtype_variant!(VariantType::Vector2Array, v),
            Vector3Array(v) => newtype_variant!(VariantType::Vector3Array, v),
            ColorArray(v) => newtype_variant!(VariantType::ColorArray, v),
        }
    }
}

struct VariantDispatchVisitor;

///Just so I can call `deserialize_identifier` instead of `deserialize_enum` to prevent MessagePack
/// from panicking...
struct VariantDispatchDiscriminant(VariantType);

struct VariantDispatchDiscriminantVisitor;

impl<'de> Visitor<'de> for VariantDispatchDiscriminantVisitor {
    type Value = VariantDispatchDiscriminant;
    
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a VariantType")
    }
    
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        use VariantType::*;
        Ok(VariantDispatchDiscriminant(
            if value == Nil as u64 { Nil }
            else if value == Bool as u64 { Bool }
            else if value == I64 as u64 { I64 }
            else if value == F64 as u64 { F64 }
            else if value == GodotString as u64 { GodotString }
            else if value == Vector2 as u64 { Vector2 }
            else if value == Rect2 as u64 { Rect2 }
            else if value == Vector3 as u64 { Vector3 }
            else if value == Transform2D as u64 { Transform2D }
            else if value == Plane as u64 { Plane }
            else if value == Quat as u64 { Quat }
            else if value == Aabb as u64 { Aabb }
            else if value == Basis as u64 { Basis }
            else if value == Transform as u64 { Transform }
            else if value == Color as u64 { Color }
            else if value == NodePath as u64 { NodePath }
            else if value == Rid as u64 { Rid }
            else if value == Object as u64 { Object }
            else if value == Dictionary as u64 { Dictionary }
            else if value == VariantArray as u64 { VariantArray }
            else if value == ByteArray as u64 { ByteArray }
            else if value == Int32Array as u64 { Int32Array }
            else if value == Float32Array as u64 { Float32Array }
            else if value == StringArray as u64 { StringArray }
            else if value == Vector2Array as u64 { Vector2Array }
            else if value == Vector3Array as u64 { Vector3Array }
            else if value == ColorArray as u64 { ColorArray }
            else {
                return Err(E::custom(&*format!("invalid VariantType value: {}", value)))
            }
        ))
    }
    
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: Error,
    {
        use VariantType::*;
        Ok(VariantDispatchDiscriminant(
            if value == Nil.name() { Nil }
            else if value == Bool.name() { Bool }
            else if value == I64.name() { I64 }
            else if value == F64.name() { F64 }
            else if value == GodotString.name() { GodotString }
            else if value == Vector2.name() { Vector2 }
            else if value == Rect2.name() { Rect2 }
            else if value == Vector3.name() { Vector3 }
            else if value == Transform2D.name() { Transform2D }
            else if value == Plane.name() { Plane }
            else if value == Quat.name() { Quat }
            else if value == Aabb.name() { Aabb }
            else if value == Basis.name() { Basis }
            else if value == Transform.name() { Transform }
            else if value == Color.name() { Color }
            else if value == NodePath.name() { NodePath }
            else if value == Rid.name() { Rid }
            else if value == Object.name() { Object }
            else if value == Dictionary.name() { Dictionary }
            else if value == VariantArray.name() { VariantArray }
            else if value == ByteArray.name() { ByteArray }
            else if value == Int32Array.name() { Int32Array }
            else if value == Float32Array.name() { Float32Array }
            else if value == StringArray.name() { StringArray }
            else if value == Vector2Array.name() { Vector2Array }
            else if value == Vector3Array.name() { Vector3Array }
            else if value == ColorArray.name() { ColorArray }
            else {
                return Err(E::custom(&*format!("invalid VariantType value: {}", value)))
            }
        ))
    }
    
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
      E: Error,
    {
        use VariantType::*;
        Ok(VariantDispatchDiscriminant(
            if value == Nil.name().as_bytes() { Nil }
            else if value == Bool.name().as_bytes() { Bool }
            else if value == I64.name().as_bytes() { I64 }
            else if value == F64.name().as_bytes() { F64 }
            else if value == GodotString.name().as_bytes() { GodotString }
            else if value == Vector2.name().as_bytes() { Vector2 }
            else if value == Rect2.name().as_bytes() { Rect2 }
            else if value == Vector3.name().as_bytes() { Vector3 }
            else if value == Transform2D.name().as_bytes() { Transform2D }
            else if value == Plane.name().as_bytes() { Plane }
            else if value == Quat.name().as_bytes() { Quat }
            else if value == Aabb.name().as_bytes() { Aabb }
            else if value == Basis.name().as_bytes() { Basis }
            else if value == Transform.name().as_bytes() { Transform }
            else if value == Color.name().as_bytes() { Color }
            else if value == NodePath.name().as_bytes() { NodePath }
            else if value == Rid.name().as_bytes() { Rid }
            else if value == Object.name().as_bytes() { Object }
            else if value == Dictionary.name().as_bytes() { Dictionary }
            else if value == VariantArray.name().as_bytes() { VariantArray }
            else if value == ByteArray.name().as_bytes() { ByteArray }
            else if value == Int32Array.name().as_bytes() { Int32Array }
            else if value == Float32Array.name().as_bytes() { Float32Array }
            else if value == StringArray.name().as_bytes() { StringArray }
            else if value == Vector2Array.name().as_bytes() { Vector2Array }
            else if value == Vector3Array.name().as_bytes() { Vector3Array }
            else if value == ColorArray.name().as_bytes() { ColorArray }
            else {
                return Err(E::custom(&*format!("invalid VariantType value: {:?}", value)))
            }
        ))
    }
}

impl<'de> Deserialize<'de> for VariantDispatchDiscriminant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de> {
        deserializer.deserialize_identifier(VariantDispatchDiscriminantVisitor)
    }
}

impl<'de> Visitor<'de> for VariantDispatchVisitor {
    type Value = VariantDispatch;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("enum VariantDispatch")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        use VariantType::*;
        let (t, v) = data.variant::<VariantDispatchDiscriminant>()?;
        Ok(match t.0 {
            Nil => {
                v.unit_variant()?;
                VariantDispatch::Nil
            }
            Bool => VariantDispatch::Bool(v.newtype_variant()?),
            I64 => VariantDispatch::I64(v.newtype_variant()?),
            F64 => VariantDispatch::F64(v.newtype_variant()?),
            GodotString => VariantDispatch::GodotString(v.newtype_variant()?),
            Vector2 => VariantDispatch::Vector2(v.newtype_variant()?),
            Rect2 => VariantDispatch::Rect2(v.newtype_variant()?),
            Vector3 => VariantDispatch::Vector3(v.newtype_variant()?),
            Transform2D => VariantDispatch::Transform2D(v.newtype_variant()?),
            Plane => VariantDispatch::Plane(v.newtype_variant()?),
            Quat => VariantDispatch::Quat(v.newtype_variant()?),
            Aabb => VariantDispatch::Aabb(v.newtype_variant()?),
            Basis => VariantDispatch::Basis(v.newtype_variant()?),
            Transform => VariantDispatch::Transform(v.newtype_variant()?),
            Color => VariantDispatch::Color(v.newtype_variant()?),
            NodePath => VariantDispatch::NodePath(v.newtype_variant()?),
            Rid => VariantDispatch::Rid(v.newtype_variant()?),
            Object => {
                // should return None
                VariantDispatch::Object(v.newtype_variant::<Option<()>>()?.to_variant())
            }
            Dictionary => VariantDispatch::Dictionary(v.newtype_variant::<DictionaryDispatch>()?.0),
            VariantArray => VariantDispatch::VariantArray(
                v.newtype_variant::<Vec<VariantDispatch>>()?
                    .iter()
                    .map(Into::<Variant>::into)
                    .collect::<variant_array::VariantArray<Unique>>()
                    .into_shared(),
            ),
            ByteArray => VariantDispatch::ByteArray(v.newtype_variant()?),
            Int32Array => VariantDispatch::Int32Array(v.newtype_variant()?),
            Float32Array => VariantDispatch::Float32Array(v.newtype_variant()?),
            StringArray => VariantDispatch::StringArray(v.newtype_variant()?),
            Vector2Array => VariantDispatch::Vector2Array(v.newtype_variant()?),
            Vector3Array => VariantDispatch::Vector3Array(v.newtype_variant()?),
            ColorArray => VariantDispatch::ColorArray(v.newtype_variant()?),
        })
    }
}

impl<'de> Deserialize<'de> for VariantDispatch {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum(
            "VariantDispatch",
            &VariantType::NAMES,
            VariantDispatchVisitor,
        )
    }
}

// struct VariantVisitor;
//
// impl<'de> Visitor<'de> for VariantVisitor {
//     type Value = Variant;
//
//     fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
//         formatter.write_str("a Variant")
//     }
//
//     fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(v.to_variant())
//     }
//
//     fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(v.to_variant())
//     }
//
//     fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         self.visit_i64(v as i64)
//     }
//
//     fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(v.to_variant())
//     }
//
//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         if v == "Nil" {
//             //`VariantDispatch::Nil` could be represented as the string "Nil"
//             return Ok(Variant::new());
//         }
//         Ok(v.to_variant())
//     }
//
//     fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(ByteArray::from_slice(v).to_variant())
//     }
//
//     fn visit_none<E>(self) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(Variant::new())
//     }
//
//     fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, <D as Deserializer<'de>>::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_any(self)
//     }
//
//     fn visit_unit<E>(self) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(().to_variant())
//     }
//
//     fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_any(VariantVisitor)
//     }
//
//     fn visit_seq<A>(self, seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
//     where
//         A: SeqAccess<'de>,
//     {
//         let arr = VariantArrayVisitor.visit_seq(seq)?;
//         let len = arr.len();
//
//         if len == 1 {
//             if let VariantDispatch::VariantArray(arr) = arr.get(0).dispatch() {
//                 if arr.len() == 3 {
//                     if let Some(v) = arr.get(0).try_to_vector3() {
//                         //TODO: Cbor actually seems to serialize [Vector3; 3] with the tag of each
//                         // element as the first 3 values in a sequence. Currently deserializing as
//                         // `[Null, Null, Null, (...), (...), (...)]`
//                         //assume format may have treated Basis as a sequence of one element
//                         if let Some(basis) = basis_seq(&arr, v) {
//                             return Ok(basis.to_variant());
//                         }
//                     }
//                 }
//             }
//         } else if len == 2 {
//             let first = arr.get(0).dispatch();
//             match first {
//                 VariantDispatch::F64(x) => {
//                     let x = x as f32;
//                     if let Some(y) = f32_field(&arr.get(1)) {
//                         return Ok(Vector2 { x, y }.to_variant());
//                     }
//                 }
//                 VariantDispatch::Vector2(position) => {
//                     if let Some(size) = arr.get(1).try_to_vector2() {
//                         return Ok(Rect2 { position, size }.to_variant());
//                     }
//                 }
//                 VariantDispatch::Vector3(pos_or_norm) => {
//                     let next = arr.get(1);
//                     if let Some(d) = f32_field(&next) {
//                         let normal = pos_or_norm;
//                         return Ok(Plane { normal, d }.to_variant());
//                     } else if let Some(size) = next.try_to_vector3() {
//                         let position = pos_or_norm;
//                         return Ok(Aabb { position, size }.to_variant());
//                     }
//                 }
//                 _ => {}
//             }
//         } else if len == 3 {
//             let first = arr.get(0).dispatch();
//             match first {
//                 VariantDispatch::F64(x) => {
//                     let x = x as f32;
//                     if let Some(y) = f32_field(&arr.get(1)) {
//                         if let Some(z) = f32_field(&arr.get(2)) {
//                             return Ok(Vector3 { x, y, z }.to_variant());
//                         }
//                     }
//                 }
//                 VariantDispatch::Vector2(x) => {
//                     if let Some(y) = arr.get(1).try_to_vector2() {
//                         if let Some(origin) = arr.get(2).try_to_vector2() {
//                             return Ok(Transform2D { x, y, origin }.to_variant());
//                         }
//                     }
//                 }
//                 VariantDispatch::Vector3(v) => {
//                     if let Some(basis) = basis_seq(&arr, v) {
//                         return Ok(basis.to_variant());
//                     }
//                 }
//                 _ => {}
//             }
//         } else if len == 4 {
//             if let Some(r) = f32_field(&arr.get(0)) {
//                 if let Some(g) = f32_field(&arr.get(1)) {
//                     if let Some(b) = f32_field(&arr.get(2)) {
//                         if let Some(a) = f32_field(&arr.get(3)) {
//                             //Assume it's a Color rather than a Quat since Godot calls arrays of
//                             //4-float structs `ColorArray`s.
//                             return Ok(Color { r, g, b, a }.to_variant());
//                         }
//                     }
//                 }
//             }
//         }
//
//         Ok(arr.owned_to_variant())
//     }
//
//     fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
//     where
//         A: MapAccess<'de>,
//     {
//         let dict = DictionaryVisitor.visit_map(map)?;
//         let len = dict.len();
//         if len == 1 {
//             let (key, value) = dict.iter().next().unwrap();
//             if let Some(key) = key.try_to_string() {
//                 if let Some(v) = string_tagged(&key, value) {
//                     return Ok(v);
//                 }
//             } else if let Some(key) = key.try_to_i64() {
//                 if let Some(v) = int_tagged(key, value) {
//                     return Ok(v);
//                 }
//             } else if let Some(arr) = key.try_to_byte_array() {
//                 if arr.read().as_slice() == b"elements" {
//                     if value.get_type() == VariantType::Basis {
//                         return Ok(value);
//                     }
//                 }
//             }
//         } else if len == 2 {
//             if let Some(v) = vec2_plane_xform_rect2_or_aabb(&dict) {
//                 return Ok(v);
//             }
//         } else if len == 3 {
//             if let Some(v) = vec3_or_xform2d(&dict) {
//                 return Ok(v);
//             }
//         } else if len == 4 {
//             if let Some(v) = quat_or_color(&dict) {
//                 return Ok(v);
//             }
//         }
//
//         //Didn't appear to be any core type, just return the dictionary.
//         Ok(dict.owned_to_variant())
//     }
// }
//
// impl<'de> Deserialize<'de> for Variant {
//     #[inline]
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_any(VariantVisitor)
//     }
// }
//
// fn basis_seq<Access: ThreadAccess>(arr: &VariantArray<Access>, first: Vector3) -> Option<Basis> {
//     if let Some(second) = arr.get(1).try_to_vector3() {
//         if let Some(third) = arr.get(2).try_to_vector3() {
//             return Some(Basis {
//                 elements: [first, second, third],
//             });
//         }
//     }
//     None
// }
//
// fn string_tagged(key: &str, value: Variant) -> Option<Variant> {
//     let s = key;
//     let value_type = value.get_type();
//     if (s == value_type.name())
//         || ((value_type == Option::<()>::None.to_variant().get_type()) && (s == "Object"))
//         || ((value_type == ().to_variant().get_type()) && (s == "Rid"))
//         //maybe a Basis represented as a Map, in which case visit_seq will have assumed [Vector3; 3] was a Basis
//         || ((value_type == VariantType::Basis) && (s == "elements"))
//     {
//         return Some(value);
//     } else if s == VariantType::NodePath.name() {
//         if let Some(path) = value.try_to_string() {
//             return Some(NodePath::from_str(&*path).to_variant());
//         }
//     } else if let Some(arr) = value.try_to_array() {
//         if let Some(s) = s.strip_suffix("Array") {
//             match s {
//                 "Variant" => return Some(value), //for completeness, should have been handled by `s == value_type.name()`
//                 "Byte" => return Some(ByteArray::from_variant_array(&arr).to_variant()),
//                 "Int32" => return Some(Int32Array::from_variant_array(&arr).to_variant()),
//                 "Float32" => return Some(Float32Array::from_variant_array(&arr).to_variant()),
//                 "Vector2" => return Some(Vector2Array::from_variant_array(&arr).to_variant()),
//                 "Vector3" => return Some(Vector3Array::from_variant_array(&arr).to_variant()),
//                 "Color" => return Some(ColorArray::from_variant_array(&arr).to_variant()),
//                 _ => {}
//             }
//         }
//     }
//     None
// }
//
// fn int_tagged(key: i64, value: Variant) -> Option<Variant> {
//     //TODO: The field enums serde generates for structs could get stored as ints.
//     // We could either hand-write all the impls so we know what the int will be,
//     // or assume serde will keep the indices the same as the declaration order,
//     // or just not support deserializing Variants from formats that store the field
//     // identifier as an int (VariantDispatch should still work).
//     let i = key;
//     if (i == value.get_type() as i64) || ((i == 0) && (value.get_type() == VariantType::Basis)) {
//         return Some(value);
//     } else if (i == VariantType::Object as i64) && (value.get_type() == VariantType::Nil) {
//         return Some(Variant::new());
//     } else if (i == VariantType::Rid as i64) && (value.get_type() == ().to_variant().get_type()) {
//         return Some(Rid::new().to_variant());
//     } else if let Some(arr) = value.try_to_vector3_array() {
//         if i == 0 && arr.len() == 3 {
//             return Some(
//                 Basis {
//                     elements: [arr.get(0), arr.get(1), arr.get(2)],
//                 }
//                 .to_variant(),
//             );
//         }
//     } else if let Some(arr) = value.try_to_array() {
//         if i == 0 {
//             if arr.len() == 3 {
//                 unsafe {
//                     let e0 = arr.get_ref(0);
//                     let e1 = arr.get_ref(1);
//                     let e2 = arr.get_ref(2);
//                     return e0
//                         .try_to_vector3()
//                         .zip(e1.try_to_vector3())
//                         .zip(e2.try_to_vector3())
//                         .map(|((e0, e1), e2)| {
//                             Basis {
//                                 elements: [e0, e1, e2],
//                             }
//                             .to_variant()
//                         });
//                 }
//             }
//         } else if i == VariantType::ByteArray as i64 {
//             return Some(ByteArray::from_variant_array(&arr).to_variant());
//         } else if i == VariantType::Int32Array as i64 {
//             return Some(Int32Array::from_variant_array(&arr).to_variant());
//         } else if i == VariantType::Float32Array as i64 {
//             return Some(Float32Array::from_variant_array(&arr).to_variant());
//         } else if i == VariantType::Vector2Array as i64 {
//             return Some(Vector2Array::from_variant_array(&arr).to_variant());
//         } else if i == VariantType::Vector3Array as i64 {
//             return Some(Vector3Array::from_variant_array(&arr).to_variant());
//         } else if i == VariantType::ColorArray as i64 {
//             return Some(ColorArray::from_variant_array(&arr).to_variant());
//         }
//     }
//     None
// }
//
// /// Struct to store possible core type field names as static Variants for optimized dictionary deserialization.
// struct Keys {
//     /// Vector2, Vector3, Transform2D, Quat
//     pub x: Variant,
//     /// Vector2, Vector3, Transform2D, Quat
//     pub y: Variant,
//     /// Vector3, Quat
//     pub z: Variant,
//     /// Quat
//     pub w: Variant,
//     /// Color
//     pub r: Variant,
//     /// Color
//     pub g: Variant,
//     /// Color
//     pub b: Variant,
//     /// Color
//     pub a: Variant,
//     /// Plane
//     pub normal: Variant,
//     /// Plane
//     pub d: Variant,
//     /// Transform
//     pub basis: Variant,
//     /// Transform, Transform2D
//     pub origin: Variant,
//     /// Rect2, Aabb
//     pub position: Variant,
//     /// Rect2, Aabb
//     pub size: Variant,
// }
//
// static KEYS: Lazy<Keys> = Lazy::new(|| Keys {
//     x: "x".to_variant(),
//     y: "y".to_variant(),
//     z: "z".to_variant(),
//     w: "w".to_variant(),
//     r: "r".to_variant(),
//     g: "g".to_variant(),
//     b: "b".to_variant(),
//     a: "a".to_variant(),
//     normal: "normal".to_variant(),
//     d: "d".to_variant(),
//     basis: "basis".to_variant(),
//     origin: "origin".to_variant(),
//     position: "position".to_variant(),
//     size: "size".to_variant(),
// });
//
// /// ["x", "y"]
// ///
// /// Used for Vector2, as well as short-circuiting Vector3 and Transform2D.
// static XY_KEYS: Lazy<VariantArray<Shared>> = Lazy::new(|| {
//     let keys = &*KEYS;
//     [&keys.x, &keys.y]
//         .iter()
//         .collect::<VariantArray<Unique>>()
//         .into_shared()
// });
//
// /// Plane field names.
// static PLANE_KEYS: Lazy<VariantArray<Shared>> = Lazy::new(|| {
//     let keys = &*KEYS;
//     [&keys.normal, &keys.d]
//         .iter()
//         .collect::<VariantArray<Unique>>()
//         .into_shared()
// });
//
// /// Transform (3D) field names.
// static XFORM_KEYS: Lazy<VariantArray<Shared>> = Lazy::new(|| {
//     let keys = &*KEYS;
//     [&keys.basis, &keys.origin]
//         .iter()
//         .collect::<VariantArray<Unique>>()
//         .into_shared()
// });
//
// /// Bounding box field names, for Rect2 and Aabb.
// static BB_KEYS: Lazy<VariantArray<Shared>> = Lazy::new(|| {
//     let keys = &*KEYS;
//     [&keys.position, &keys.size]
//         .iter()
//         .collect::<VariantArray<Unique>>()
//         .into_shared()
// });
//
// /// `dict` has been verified to contain 2 entries.
// ///
// /// This function checks the keys to determine which Variant type to return, and verifies the types
// /// of the values before copying them into the correct core type and returning it as a Variant.
// fn vec2_plane_xform_rect2_or_aabb(dict: &Dictionary<Unique>) -> Option<Variant> {
//     let keys = &*KEYS;
//     unsafe {
//         //SAFETY: `dict` is Unique, so it shouldn't be modified through another reference,
//         // we verify that all keys exist before calling `get_ref`, so the dictionary won't re-allocate,
//         // and we won't be returning any references, so they can't be invalidated later.
//         if dict.contains_all(&*XY_KEYS) {
//             get_f32(dict, &keys.x)
//                 .zip(get_f32(dict, &keys.y))
//                 .map(|(x, y)| Vector2 { x, y }.to_variant())
//         } else if dict.contains_all(&*PLANE_KEYS) {
//             dict.get_ref(&keys.normal)
//                 .try_to_vector3()
//                 .zip(get_f32(dict, &keys.d))
//                 .map(|(normal, d)| Plane { normal, d }.to_variant())
//         } else if dict.contains_all(&*XFORM_KEYS) {
//             dict.get_ref(&keys.basis)
//                 .try_to_basis()
//                 .zip(dict.get_ref(&keys.origin).try_to_vector3())
//                 .map(|(basis, origin)| Transform { basis, origin }.to_variant())
//         } else if dict.contains_all(&*BB_KEYS) {
//             match dict.get_ref(&keys.position).dispatch() {
//                 VariantDispatch::Vector2(position) => dict
//                     .get_ref(&keys.size)
//                     .try_to_vector2()
//                     .map(|size| Rect2 { position, size }.to_variant()),
//                 VariantDispatch::Vector3(position) => dict
//                     .get_ref(&keys.size)
//                     .try_to_vector3()
//                     .map(|size| Aabb { position, size }.to_variant()),
//                 _ => None,
//             }
//         } else {
//             None
//         }
//     }
// }
//
// /// `dict` has been verified to contain 3 entries.
// ///
// /// This function checks the keys to determine which Variant type to return, and verifies the types
// /// of the values before copying them into the correct core type and converting it to a Variant.
// fn vec3_or_xform2d(dict: &Dictionary<Unique>) -> Option<Variant> {
//     if dict.contains_all(&*XY_KEYS) {
//         let keys = &*KEYS;
//         unsafe {
//             //SAFETY: `dict` is Unique, so it shouldn't be modified through another reference,
//             // we verify that all keys exist before calling `get_ref`, so the dictionary won't re-allocate,
//             // and we won't be returning any references, so they can't be invalidated later.
//             if dict.contains(&keys.z) {
//                 get_f32(dict, &keys.x)
//                     .zip(get_f32(dict, &keys.y))
//                     .zip(get_f32(dict, &keys.z))
//                     .map(|((x, y), z)| Vector3 { x, y, z }.to_variant())
//             } else if dict.contains(&keys.origin) {
//                 dict.get_ref(&keys.x)
//                     .try_to_vector2()
//                     .zip(dict.get_ref(&keys.y).try_to_vector2())
//                     .zip(dict.get_ref(&keys.origin).try_to_vector2())
//                     .map(|((x, y), origin)| Transform2D { x, y, origin }.to_variant())
//             } else {
//                 None
//             }
//         }
//     } else {
//         None
//     }
// }
//
// /// Quat field names.
// static QUAT_KEYS: Lazy<VariantArray<Shared>> = Lazy::new(|| {
//     let keys = &*KEYS;
//     [&keys.x, &keys.y, &keys.z, &keys.w]
//         .iter()
//         .collect::<VariantArray<Unique>>()
//         .into_shared()
// });
//
// /// Color field names
// static COLOR_KEYS: Lazy<VariantArray<Shared>> = Lazy::new(|| {
//     let keys = &*KEYS;
//     [&keys.r, &keys.g, &keys.b, &keys.a]
//         .iter()
//         .collect::<VariantArray<Unique>>()
//         .into_shared()
// });
//
// /// `dict` has been verified to contain 4 entries.
// ///
// /// This function checks the keys to determine which Variant type to return, and verifies the types
// /// of the values before copying them into the correct core type and converting it to a Variant.
// fn quat_or_color(dict: &Dictionary<Unique>) -> Option<Variant> {
//     unsafe {
//         //SAFETY: `dict` is Unique, so it shouldn't be modified through another reference,
//         // we verify that all keys exist before calling `get_ref`, so the dictionary won't re-allocate,
//         // and we won't be returning any references, so they can't be invalidated later.
//         if dict.contains_all(&*QUAT_KEYS) {
//             let keys = &*KEYS;
//             get_f32(dict, &keys.x)
//                 .zip(get_f32(dict, &keys.y))
//                 .zip(get_f32(dict, &keys.z))
//                 .zip(get_f32(dict, &keys.w))
//                 .map(|(((x, y), z), w)| Quat { x, y, z, w }.to_variant())
//         } else if dict.contains_all(&*COLOR_KEYS) {
//             let keys = &*KEYS;
//             get_f32(dict, &keys.r)
//                 .zip(get_f32(dict, &keys.g))
//                 .zip(get_f32(dict, &keys.b))
//                 .zip(get_f32(dict, &keys.a))
//                 .map(|(((r, g), b), a)| Color { r, g, b, a }.to_variant())
//         } else {
//             None
//         }
//     }
// }
//
// /// Get the value corresponding to `key` as an `f32` if it's a number.
// ///
// /// # Safety
// /// This calls `Dictionary::get_ref`, so either `key` must exist, or there must be no other
// /// references to values in this dictionary still in use, otherwise Godot may re-allocate the
// /// dictionary and invalidate any other references.
// unsafe fn get_f32(dict: &Dictionary<Unique>, key: &Variant) -> Option<f32> {
//     f32_field(dict.get_ref(&key))
// }
//
// /// Tries to cast the value to an f32 first by checking if it's an f64, then checking if it's an i64
// /// (so users and formats can leave the `.0` off of whole-number floats.
// fn f32_field(v: &Variant) -> Option<f32> {
//     v.try_to_f64()
//         .map(|f| f as f32)
//         .or_else(|| v.try_to_i64().map(|i| i as f32))
// }
