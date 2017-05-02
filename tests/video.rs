extern crate sdl2;

#[test]
fn display_name_no_segfault() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // hopefully no one has a 100 screen to see this test pass
    let r = video_subsystem.display_name(99);
    assert!(r.is_err());
}
