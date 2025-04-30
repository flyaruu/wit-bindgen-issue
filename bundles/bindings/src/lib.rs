// mod expanded;

pub mod api {
    wit_bindgen::generate!({
        generate_all,
        world: "superhero",
        pub_export_macro: true,
    });
}
