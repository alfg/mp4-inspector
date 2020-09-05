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
    boxes.push(build_box(&m.moov.mvhd));

    // trak.
    for track in m.tracks().iter() {
        boxes.push(build_box(&track.trak));
        boxes.push(build_box(&track.trak.tkhd));
        if let Some(ref edts) = track.trak.edts {
            boxes.push(build_box(edts));
            if let Some(ref elst) = edts.elst {
                boxes.push(build_box(elst));
            }
        }

        // trak.mdia
        let mdia = &track.trak.mdia;
        boxes.push(build_box(mdia));
        boxes.push(build_box(&mdia.mdhd));
        boxes.push(build_box(&mdia.hdlr));
        boxes.push(build_box(&track.trak.mdia.minf));

        // trak.mdia.minf
        let minf = &track.trak.mdia.minf;
        if let Some(ref vmhd) = &minf.vmhd {
            boxes.push(build_box(vmhd));
        }
        if let Some(ref smhd) = &minf.smhd {
            boxes.push(build_box(smhd));
        }

        // trak.mdia.minf.stbl
        let stbl = &track.trak.mdia.minf.stbl;
        boxes.push(build_box(stbl));
        boxes.push(build_box(&stbl.stsd));
        if let Some(ref avc1) = &stbl.stsd.avc1 {
            boxes.push(build_box(avc1));
        }
        if let Some(ref hev1) = &stbl.stsd.hev1 {
            boxes.push(build_box(hev1));
        }
        if let Some(ref mp4a) = &stbl.stsd.mp4a {
            boxes.push(build_box(mp4a));
        }
        boxes.push(build_box(&stbl.stts));
        if let Some(ref ctts) = &stbl.ctts {
            boxes.push(build_box(ctts));
        }
        if let Some(ref stss) = &stbl.stss {
            boxes.push(build_box(stss));
        }
        boxes.push(build_box(&stbl.stsc));
        boxes.push(build_box(&stbl.stsz));
        if let Some(ref stco) = &stbl.stco {
            boxes.push(build_box(stco));
        }
        if let Some(ref co64) = &stbl.co64 {
            boxes.push(build_box(co64));
        }
    }

    // If fragmented, add moof boxes.
    for moof in m.moofs.iter() {
        boxes.push(build_box(moof));
        boxes.push(build_box(&moof.mfhd));
        for traf in moof.trafs.iter() {
            boxes.push(build_box(traf));
            boxes.push(build_box(&traf.tfhd));
        }
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
    log("wasm: get media_info");

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