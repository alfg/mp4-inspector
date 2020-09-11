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
    log("wasm: get_boxes");

    let c = Cursor::new(buf);
    let len = buf.len() as u64;
    let m = mp4::Mp4Reader::read_header(c, len).unwrap();

    let mut boxes = Vec::new();
    boxes.push(build_box(&m.ftyp));
    boxes.push(build_box(&m.moov));

    // If fragmented, add moof boxes.
    for moof in m.moofs.iter() {
        boxes.push(build_box(moof));
    }

    JsValue::from_serde(&boxes).unwrap()
}

#[wasm_bindgen]
pub fn get_tracks(buf: &[u8]) -> JsValue {
    log("wasm: get_tracks");

    let c = Cursor::new(buf);
    let len = buf.len() as u64;
    let m = mp4::Mp4Reader::read_header(c, len).unwrap();

    let mut tracks = Vec::new();
    for track in m.tracks().iter() {
        let media_info = match track.track_type().unwrap() {
            mp4::TrackType::Video => video_info(track),
            mp4::TrackType::Audio => audio_info(track),
            mp4::TrackType::Subtitle => subtitle_info(track),
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

#[wasm_bindgen]
pub fn get_media_info(buf: &[u8]) -> JsValue {
    log("wasm: get_media_info");

    let c = Cursor::new(buf);
    let len = buf.len() as u64;
    let m = mp4::Mp4Reader::read_header(c, len).unwrap();

    let mut compatible_brands = String::new();
    for brand in m.compatible_brands().iter() {
        compatible_brands.push_str(&brand.to_string());
        compatible_brands.push_str(" ");
    }

    let media_info = MediaInfo{
        size: m.size(),
        major_brand: m.major_brand().to_string(),
        minor_version: m.minor_version(),
        compatible_brands: compatible_brands,
        duration: m.duration().as_millis(),
        timescale: m.timescale(),
        fragmented: m.is_fragmented(),
    };
    JsValue::from_serde(&media_info).unwrap()
}

#[wasm_bindgen]
pub fn get_samples(buf: &[u8]) -> JsValue {
    log("wasm: get_samples");

    let c = Cursor::new(buf);
    let len = buf.len() as u64;
    let mut m = mp4::Mp4Reader::read_header(c, len).unwrap();

    let mut resp = Vec::new();
    
    for track_idx in 0..m.tracks().len() {
        let track_id = track_idx as u32 + 1;
        let sample_count = m.sample_count(track_id).unwrap();

        let track_type = m.tracks()[track_idx].track_type().unwrap();
        let box_type = m.tracks()[track_idx].box_type().unwrap();


        let mut samples = Vec::new();

        for sample_idx in 0..sample_count {
            let sample_id = sample_idx + 1;
            let sample = m.read_sample(track_id, sample_id);


            if let Some(ref samp) = sample.unwrap() {
                let s = Sample{
                    start_time: samp.start_time,
                    duration: samp.duration,
                    rendering_offset: samp.rendering_offset,
                    is_sync: samp.is_sync,
                };
                samples.push(s);
            }
        }
        resp.push(Samples{
            track_id,
            track_type: track_type.to_string(),
            box_type: box_type.to_string(),
            samples
        });
    }
    JsValue::from_serde(&resp).unwrap()
}

#[derive(Serialize)]
pub struct Samples {
    track_id: u32,
    track_type: String,
    box_type: String,
    samples: Vec<Sample>,
}

#[derive(Serialize)]
pub struct Sample {
    pub start_time: u64,
    pub duration: u32,
    pub rendering_offset: i32,
    pub is_sync: bool,
}

#[derive(Serialize)]
pub struct Box {
    pub name: String,
    pub size: u64,
    pub json: String,
}

fn build_box<M: Mp4Box>(ref m: &M) -> Box {
    return Box{
        name: m.box_type().to_string(),
        size: m.box_size(),
        json: m.to_json().unwrap(),
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
    if track.trak.mdia.minf.stbl.stsd.avc1.is_some() {
        Ok(format!(
            "{} ({}) ({:?}), {}x{}, {} kb/s, {:.2} fps",
            track.media_type()?,
            track.video_profile()?,
            track.box_type()?,
            track.width(),
            track.height(),
            track.bitrate() / 1000,
            track.frame_rate()
        ))
    } else {
        Ok(format!(
            "{} ({:?}), {}x{}, {} kb/s, {:.2} fps",
            track.media_type()?,
            track.box_type()?,
            track.width(),
            track.height(),
            track.bitrate() / 1000,
            track.frame_rate()
        ))
    }
}

fn audio_info(track: &Mp4Track) -> Result<String> {
    if let Some(ref mp4a) = track.trak.mdia.minf.stbl.stsd.mp4a {
        if mp4a.esds.is_some() {
            let profile = match track.audio_profile() {
                Ok(val) => val.to_string(),
                _ => "-".to_string(),
            };

            let channel_config = match track.channel_config() {
                Ok(val) => val.to_string(),
                _ => "-".to_string(),
            };

            Ok(format!(
                "{} ({}) ({:?}), {} Hz, {}, {} kb/s",
                track.media_type()?,
                profile,
                track.box_type()?,
                track.sample_freq_index()?.freq(),
                channel_config,
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

fn subtitle_info(track: &Mp4Track) -> Result<String> {
    if track.trak.mdia.minf.stbl.stsd.tx3g.is_some() {
        Ok(format!(
            "{} ({:?})",
            track.media_type()?,
            track.box_type()?,
        ))
    } else {
        Err(Error::InvalidData("tx3g box not found"))
    }
}

#[derive(Serialize)]
pub struct MediaInfo {
    pub size: u64,
    pub major_brand: String,
    pub minor_version: u32,
    pub compatible_brands: String,
    pub timescale: u32,
    pub duration: u128,
    pub fragmented: bool,
}