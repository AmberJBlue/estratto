wai_bindgen_rust::export!("estratto.wai");

pub struct Estratto;

impl estratto::Estratto for Estratto {
    fn hello() -> String {
        return "Hello, estratto!".to_owned();
    }
}
