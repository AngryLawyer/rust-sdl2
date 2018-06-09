/* automatically generated by rust-bindgen */

pub const MIX_MAJOR_VERSION: u32 = 2;
pub const MIX_MINOR_VERSION: u32 = 0;
pub const MIX_PATCHLEVEL: u32 = 2;
pub const MIX_CHANNELS: u32 = 8;
pub const MIX_DEFAULT_FREQUENCY: u32 = 22050;
pub const MIX_DEFAULT_FORMAT: u32 = 32784;
pub const MIX_DEFAULT_CHANNELS: u32 = 2;
pub const MIX_MAX_VOLUME: u32 = 128;
pub const MIX_CHANNEL_POST: i32 = -2;
pub const MIX_EFFECTSMAXSPEED: &'static [u8; 20usize] = b"MIX_EFFECTSMAXSPEED\0";
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type Uint8 = u8;
pub type Sint16 = i16;
pub type Uint16 = u16;
pub type Uint32 = u32;
pub type Sint64 = i64;
extern "C" {
    pub fn Mix_Linked_Version() -> *const SDL_version;
}
pub const MIX_InitFlags_MIX_INIT_FLAC: MIX_InitFlags = 1;
pub const MIX_InitFlags_MIX_INIT_MOD: MIX_InitFlags = 2;
pub const MIX_InitFlags_MIX_INIT_MP3: MIX_InitFlags = 8;
pub const MIX_InitFlags_MIX_INIT_OGG: MIX_InitFlags = 16;
pub const MIX_InitFlags_MIX_INIT_MID: MIX_InitFlags = 32;
pub type MIX_InitFlags = u32;
extern "C" {
    pub fn Mix_Init(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_Quit();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mix_Chunk {
    pub allocated: ::std::os::raw::c_int,
    pub abuf: *mut Uint8,
    pub alen: Uint32,
    pub volume: Uint8,
}
#[test]
fn bindgen_test_layout_Mix_Chunk() {
    assert_eq!(
        ::std::mem::size_of::<Mix_Chunk>(),
        24usize,
        concat!("Size of: ", stringify!(Mix_Chunk))
    );
    assert_eq!(
        ::std::mem::align_of::<Mix_Chunk>(),
        8usize,
        concat!("Alignment of ", stringify!(Mix_Chunk))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).allocated as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(allocated)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).abuf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(abuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).alen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(alen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).volume as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(volume)
        )
    );
}
pub const Mix_Fading_MIX_NO_FADING: Mix_Fading = 0;
pub const Mix_Fading_MIX_FADING_OUT: Mix_Fading = 1;
pub const Mix_Fading_MIX_FADING_IN: Mix_Fading = 2;
pub type Mix_Fading = u32;
pub const Mix_MusicType_MUS_NONE: Mix_MusicType = 0;
pub const Mix_MusicType_MUS_CMD: Mix_MusicType = 1;
pub const Mix_MusicType_MUS_WAV: Mix_MusicType = 2;
pub const Mix_MusicType_MUS_MOD: Mix_MusicType = 3;
pub const Mix_MusicType_MUS_MID: Mix_MusicType = 4;
pub const Mix_MusicType_MUS_OGG: Mix_MusicType = 5;
pub const Mix_MusicType_MUS_MP3: Mix_MusicType = 6;
pub const Mix_MusicType_MUS_MP3_MAD_UNUSED: Mix_MusicType = 7;
pub const Mix_MusicType_MUS_FLAC: Mix_MusicType = 8;
pub const Mix_MusicType_MUS_MODPLUG_UNUSED: Mix_MusicType = 9;
pub type Mix_MusicType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mix_Music {
    _unused: [u8; 0],
}
pub type Mix_Music = _Mix_Music;
extern "C" {
    pub fn Mix_OpenAudio(
        frequency: ::std::os::raw::c_int,
        format: Uint16,
        channels: ::std::os::raw::c_int,
        chunksize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_OpenAudioDevice(
        frequency: ::std::os::raw::c_int,
        format: Uint16,
        channels: ::std::os::raw::c_int,
        chunksize: ::std::os::raw::c_int,
        device: *const ::std::os::raw::c_char,
        allowed_changes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_AllocateChannels(numchans: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_QuerySpec(
        frequency: *mut ::std::os::raw::c_int,
        format: *mut Uint16,
        channels: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_LoadWAV_RW(src: *mut SDL_RWops, freesrc: ::std::os::raw::c_int) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_LoadMUS(file: *const ::std::os::raw::c_char) -> *mut Mix_Music;
}
extern "C" {
    pub fn Mix_LoadMUS_RW(src: *mut SDL_RWops, freesrc: ::std::os::raw::c_int) -> *mut Mix_Music;
}
extern "C" {
    pub fn Mix_LoadMUSType_RW(
        src: *mut SDL_RWops,
        type_: Mix_MusicType,
        freesrc: ::std::os::raw::c_int,
    ) -> *mut Mix_Music;
}
extern "C" {
    pub fn Mix_QuickLoad_WAV(mem: *mut Uint8) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_QuickLoad_RAW(mem: *mut Uint8, len: Uint32) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_FreeChunk(chunk: *mut Mix_Chunk);
}
extern "C" {
    pub fn Mix_FreeMusic(music: *mut Mix_Music);
}
extern "C" {
    pub fn Mix_GetNumChunkDecoders() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetChunkDecoder(index: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_HasChunkDecoder(name: *const ::std::os::raw::c_char) -> SDL_bool;
}
extern "C" {
    pub fn Mix_GetNumMusicDecoders() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetMusicDecoder(index: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_HasMusicDecoder(name: *const ::std::os::raw::c_char) -> SDL_bool;
}
extern "C" {
    pub fn Mix_GetMusicType(music: *const Mix_Music) -> Mix_MusicType;
}
extern "C" {
    pub fn Mix_SetPostMix(
        mix_func: ::std::option::Option<
            unsafe extern "C" fn(
                udata: *mut ::std::os::raw::c_void,
                stream: *mut Uint8,
                len: ::std::os::raw::c_int,
            ),
        >,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Mix_HookMusic(
        mix_func: ::std::option::Option<
            unsafe extern "C" fn(
                udata: *mut ::std::os::raw::c_void,
                stream: *mut Uint8,
                len: ::std::os::raw::c_int,
            ),
        >,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Mix_HookMusicFinished(music_finished: ::std::option::Option<unsafe extern "C" fn()>);
}
extern "C" {
    pub fn Mix_GetMusicHookData() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Mix_ChannelFinished(
        channel_finished: ::std::option::Option<
            unsafe extern "C" fn(channel: ::std::os::raw::c_int),
        >,
    );
}
pub type Mix_EffectFunc_t = ::std::option::Option<
    unsafe extern "C" fn(
        chan: ::std::os::raw::c_int,
        stream: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        udata: *mut ::std::os::raw::c_void,
    ),
>;
pub type Mix_EffectDone_t = ::std::option::Option<
    unsafe extern "C" fn(chan: ::std::os::raw::c_int, udata: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Mix_RegisterEffect(
        chan: ::std::os::raw::c_int,
        f: Mix_EffectFunc_t,
        d: Mix_EffectDone_t,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_UnregisterEffect(
        channel: ::std::os::raw::c_int,
        f: Mix_EffectFunc_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_UnregisterAllEffects(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetPanning(
        channel: ::std::os::raw::c_int,
        left: Uint8,
        right: Uint8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetPosition(
        channel: ::std::os::raw::c_int,
        angle: Sint16,
        distance: Uint8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetDistance(
        channel: ::std::os::raw::c_int,
        distance: Uint8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetReverseStereo(
        channel: ::std::os::raw::c_int,
        flip: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_ReserveChannels(num: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupChannel(
        which: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupChannels(
        from: ::std::os::raw::c_int,
        to: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupAvailable(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupCount(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupOldest(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupNewer(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PlayChannelTimed(
        channel: ::std::os::raw::c_int,
        chunk: *mut Mix_Chunk,
        loops: ::std::os::raw::c_int,
        ticks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PlayMusic(
        music: *mut Mix_Music,
        loops: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeInMusic(
        music: *mut Mix_Music,
        loops: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeInMusicPos(
        music: *mut Mix_Music,
        loops: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
        position: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeInChannelTimed(
        channel: ::std::os::raw::c_int,
        chunk: *mut Mix_Chunk,
        loops: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
        ticks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_Volume(
        channel: ::std::os::raw::c_int,
        volume: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_VolumeChunk(
        chunk: *mut Mix_Chunk,
        volume: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_VolumeMusic(volume: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_HaltChannel(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_HaltGroup(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_HaltMusic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_ExpireChannel(
        channel: ::std::os::raw::c_int,
        ticks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeOutChannel(
        which: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeOutGroup(
        tag: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeOutMusic(ms: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadingMusic() -> Mix_Fading;
}
extern "C" {
    pub fn Mix_FadingChannel(which: ::std::os::raw::c_int) -> Mix_Fading;
}
extern "C" {
    pub fn Mix_Pause(channel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Mix_Resume(channel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Mix_Paused(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PauseMusic();
}
extern "C" {
    pub fn Mix_ResumeMusic();
}
extern "C" {
    pub fn Mix_RewindMusic();
}
extern "C" {
    pub fn Mix_PausedMusic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetMusicPosition(position: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_Playing(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PlayingMusic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetMusicCMD(command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetSynchroValue(value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetSynchroValue() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetSoundFonts(paths: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetSoundFonts() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_EachSoundFont(
        function: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_char,
                arg2: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetChunk(channel: ::std::os::raw::c_int) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_CloseAudio();
}
