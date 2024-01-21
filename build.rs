extern crate embed_resource;
extern crate winresource;

fn main() {
    embed_resource::compile("keybind-actions.rc", embed_resource::NONE);
    
    let res = winresource::WindowsResource::new();
    res.compile().unwrap();
}
