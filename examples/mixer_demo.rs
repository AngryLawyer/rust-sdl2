/// Demonstrates the simultaneous mixing of music and sound effects.

extern crate sdl2;

use std::env;
use std::path::Path;
use sdl2::mixer::{DEFAULT_CHANNELS, INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG,
                  INIT_OGG, AUDIO_S16LSB};

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        demo(Path::new(&args[1]), Path::new(""));
    } else if args.len() == 3 {
        demo(Path::new(&args[1]), Path::new(&args[2]));
    } else {
        println!("Usage: ./demo music.[mp3|wav|ogg] [sound-effect.[mp3|wav|ogg]]")
    }
}

fn demo(music_file: &Path, sound_file: &Path) {

    println!("linked version: {}", sdl2::mixer::get_linked_version());

    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();
    let mut timer = sdl.timer().unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
                                          INIT_MODPLUG |
                                          INIT_OGG)
                            .unwrap();

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(4);

    {
        let n = sdl2::mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2::mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        }
    }

    println!("query spec => {:?}", sdl2::mixer::query_spec());

    let music = sdl2::mixer::Music::from_file(music_file).unwrap();

    fn hook_finished() {
        println!("play ends! from rust cb");
    }

    sdl2::mixer::Music::hook_finished(hook_finished);

    println!("music => {:?}", music);
    println!("music type => {:?}", music.get_type());
    println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
    println!("play => {:?}", music.play(1));

    let sound_chunk_res = sdl2::mixer::Chunk::from_file(sound_file);

    match sound_chunk_res {
        Ok(sound_chunk) => {
            println!("chunk volume => {:?}", sound_chunk.get_volume());
            println!("playing sound twice");
            let play_res = sdl2::mixer::Channel::all().play(&sound_chunk, 1);

            timer.delay(5000);

            match play_res {
                Ok(_) => { println!("played sound")},
                Err(e) => { println!("{:?}", e) },
            }
        }
        Err(e) => { println!("Cannot load sound file: {:?}", e) },
    }

    timer.delay(10000);

    println!("fading out ... {:?}", sdl2::mixer::Music::fade_out(4000));

    timer.delay(5000);

    println!("fading in from pos ... {:?}",
             music.fade_in_from_pos(1, 10000, 100.0));

    timer.delay(5000);
    sdl2::mixer::Music::halt();
    timer.delay(1000);

    println!("quitting sdl");
}
