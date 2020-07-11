use crate::api::*;
use crate::GeneratorResult;

use proc_macro2::TokenStream;
use quote::quote;

use std::io::Write;

pub fn class_doc_link(class: &GodotClass) -> String {
    format!("[{name}](struct.{name}.html)", name = class.name)
}

pub fn official_doc_url(class: &GodotClass) -> String {
    format!(
        "https://godot.readthedocs.io/en/latest/classes/class_{lower_case}.html",
        lower_case = class.name.to_lowercase(),
    )
}

pub fn generate_class_documentation(api: &Api, class: &GodotClass) -> TokenStream {
    let has_parent = class.base_class != "";
    let singleton_str = if class.singleton { "singleton " } else { "" };
    let ownership_type = if class.is_refcounted() {
        "reference counted"
    } else {
        "unsafe"
    };

    let summary_doc = if &class.name == "Reference" {
        "Base class of all reference-counted types. Inherits `Object`.".into()
    } else if &class.name == "Object" {
        "The base class of most Godot classes.".into()
    } else if has_parent {
        format!(
            "`{api_type} {singleton}class {name}` inherits `{base_class}` ({ownership_type}).",
            api_type = class.api_type,
            name = class.name,
            base_class = class.base_class,
            ownership_type = ownership_type,
            singleton = singleton_str
        )
    } else {
        format!(
            "`{api_type} {singleton}class {name}` ({ownership_type}).",
            api_type = class.api_type,
            name = class.name,
            ownership_type = ownership_type,
            singleton = singleton_str,
        )
    };

    let official_docs = format!(
        r#"## Official documentation

See the [documentation of this class]({url}) in the Godot engine's official documentation."#,
        url = official_doc_url(class),
    );

    let memory_management_docs = if class.is_refcounted() {
        r#"## Memory management

The lifetime of this object is automatically managed through reference counting."#
            .to_string()
    } else if class.instantiable {
        format!(
            r#"## Memory management

Non reference counted objects such as the ones of this type are usually owned by the engine.

`{name}` is a reference-only type. Persistent references can
only exist in the unsafe `Ref<{name}>` form.

In the cases where Rust code owns an object of this type, for example if the object was just
created on the Rust side and not passed to the engine yet, ownership should be either given
to the engine or the object must be manually destroyed using [`Ref::free`], or [`Ref::queue_free`] 
if it is a [`Node`].

[`Node`]: struct.Node.html
[`Ref::free`]: ../prelude/struct.Ref.html#method.free
[`Ref::queue_free`]: ../prelude/struct.Ref.html#method.queue_free"#,
            name = class.name
        )
    } else {
        "".into()
    };

    let base_class_docs = if class.base_class != "" {
        let mut docs = vec![];
        write!(
            &mut docs,
            r#"
## Class hierarchy

{name} inherits methods from:
"#,
            name = class.name,
        )
        .unwrap();

        list_base_classes(&mut docs, api, &class.base_class).unwrap();
        docs
    } else {
        vec![]
    };
    let base_class_docs = std::str::from_utf8(&base_class_docs).unwrap();

    let tools_docs = if class.api_type == "tools" {
        r#"
## Tool

This class is used to interact with Godot's editor."#
    } else {
        ""
    };

    let safety_doc = r#"
## Safety

All types in the Godot API have "interior mutability" in Rust parlance.
To enforce that the official [thread-safety guidelines][thread-safety] are
followed, the typestate pattern is used in the [`Ref`] and [`TRef`] smart pointers,
and the [`Instance`] API. The typestate `Access` in these types tracks whether the
access is unique, shared, or exclusive to the current thread. For more information,
see the type-level documentation on [`Ref`].

[`Ref`]: ../prelude/struct.Ref.html
[`TRef`]: ../prelude/struct.TRef.html
[`Instance`]: ../prelude/struct.Instance.html
[thread-safety]: https://docs.godotengine.org/en/stable/tutorials/threads/thread_safe_apis.html"#;

    quote! {
        #[doc=#summary_doc]
        #[doc=#official_docs]
        #[doc=#memory_management_docs]
        #[doc=#base_class_docs]
        #[doc=#tools_docs]
        #[doc=#safety_doc]
    }
}

fn list_base_classes(output: &mut impl Write, api: &Api, parent_name: &str) -> GeneratorResult {
    if let Some(parent) = api.find_class(parent_name) {
        let class_link = class_doc_link(&parent);

        writeln!(output, " - {}", class_link)?;

        if parent.base_class != "" {
            list_base_classes(output, api, &parent.base_class)?;
        }
    }

    Ok(())
}
