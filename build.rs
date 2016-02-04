use std::env;

extern crate gcc;

fn main() {
  gcc::compile_library("libcimgui.a", &[
    // cimgui
    "cimgui/cimgui/cimgui.cpp", 
    "cimgui/cimgui/drawList.cpp",
    "cimgui/cimgui/fontAtlas.cpp",
    // imgui
    "cimgui/imgui/imgui.cpp",
    "cimgui/imgui/imgui_demo.cpp",
    "cimgui/imgui/imgui_draw.cpp"
  ]);
}

