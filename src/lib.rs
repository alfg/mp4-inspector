use mp4::{Mp4Track, Mp4Box, Result, Error};
use std::io::{Cursor};
use wasm_bindgen::prelude::*;
use mp4;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: String);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u64(a: u64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a: u8);
}

extern crate web_sys;

#[wasm_bindgen]
pub fn logger(s: &str) {
    log(s);
}

#[wasm_bindgen]
pub fn get_boxes(buf: &[u8]) -> JsValue {
    log("wasm: parsing");

    let c = Cursor::new(buf);
    let len = buf.len() as u64;
    let m = mp4::Mp4Reader::read_header(c, len).unwrap();

    let mut boxes = Vec::new();
    boxes.push(build_box(&m.ftyp));
    boxes.push(build_box(&m.moov));
    boxes.push(build_box(&m.moov.mvhd));

    for track in m.tracks().iter() {
        boxes.push(build_box(&track.trak));
        boxes.push(build_box(&track.trak.tkhd));
    }

    JsValue::from_serde(&boxes).unwrap()
}

#[wasm_bindgen]
pub fn get_tracks(buf: &[u8]) -> JsValue {
    log("wasm: get tracks");

    let c = Cursor::new(buf);
    let len = buf.len() as u64;
    let m = mp4::Mp4Reader::read_header(c, len).unwrap();

    let mut tracks = Vec::new();
    for track in m.tracks().iter() {
        let media_info = match track.track_type().unwrap() {
            mp4::TrackType::Video => video_info(track),
            mp4::TrackType::Audio => audio_info(track),
        };
        let t = Track{
            id: track.track_id(),
            language: track.language().to_string(),
            track_type: track.track_type().unwrap().to_string(),
            box_type: track.box_type().unwrap().to_string(),
            media_info: media_info.unwrap(),
        };
        tracks.push(t);
    }
    JsValue::from_serde(&tracks).unwrap()
}

#[derive(Serialize)]
pub struct Box {
    pub name: String,
    pub size: u64,
}

fn build_box<M: Mp4Box + std::fmt::Debug>(ref m: &M) -> Box {
    return Box{
        name: m.box_type().to_string(),
        size: m.box_size(),
    };
}

#[derive(Serialize)]
pub struct Track {
    pub id: u32,
    pub language: String,
    pub track_type: String,
    pub box_type: String,
    pub media_info: String,
}

fn video_info(track: &Mp4Track) -> Result<String> {
    Ok(format!(
        "{} ({}) ({:?}), {}x{}, {} kb/s, {:.2} fps",
        track.media_type()?,
        track.video_profile()?,
        track.box_type()?,
        track.width(),
        track.height(),
        track.bitrate() / 1000,
        track.frame_rate_f64()
    ))
}

fn audio_info(track: &Mp4Track) -> Result<String> {
    if let Some(ref mp4a) = track.trak.mdia.minf.stbl.stsd.mp4a {
        if mp4a.esds.is_some() {
            Ok(format!(
                "{} ({}) ({:?}), {} Hz, {}, {} kb/s",
                track.media_type()?,
                track.audio_profile()?,
                track.box_type()?,
                track.sample_freq_index()?.freq(),
                track.channel_config()?,
                track.bitrate() / 1000
            ))
        } else {
            Ok(format!(
                "{} ({:?}), {} kb/s",
                track.media_type()?,
                track.box_type()?,
                track.bitrate() / 1000
            ))
        }
    } else {
        Err(Error::InvalidData("mp4a box not found"))
    }
}