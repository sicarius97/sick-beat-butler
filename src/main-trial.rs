mod sources;
mod types;
// use sources::youtube::ytdl;
// use tokio::io::{BufStream, AsyncReadExt};
use std::collections::VecDeque;
use std::process::ChildStdout;
use std::time::Duration;
// use std::io::{BufReader, BufRead};
use rodio::{Sample, Source};
use rodio::{Decoder, OutputStream, Sink};
use rodio::buffer::SamplesBuffer;
use symphonia::core::audio::{SampleBuffer, SignalSpec, Channels};
use std::io::{Cursor, Read, IoSliceMut, BufReader, Write};
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use std::process::{Child, Command, Stdio};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL, CodecType};
use symphonia::core::errors::{Error, Result as SymphResult};
use symphonia::core::formats::{Cue, FormatOptions, FormatReader, SeekMode, SeekTo, Track};
use symphonia::core::meta::{ColorMode, MetadataOptions, MetadataRevision, Tag, Value, Visual};
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::core::units::{Time, TimeBase};
use std::thread;
use std::{
    sync::{Arc, Mutex},
};
/* 
pub struct Track<P>
where
    P: AsRef<IntoIter<ChildStdout>> + Send + Sync
{
    pipe: P
}

impl<P: AsRef<IntoIter<ChildStdout>> + Iterator + Send + Sync> Source for Track<P> {
    fn current_frame_len(&self) -> Option<usize> {
        let taken_stdout = self.pipe.stdout.take();

        Some(taken_stdout.count())
    }
}
*/

const NEWLINE_BYTE: u8 = 0xA;


fn main() -> Result<(), ()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    /* 
    let ytdl_args = [
        "-j",            // print JSON information for video for metadata
        "-q",            // don't print progress logs (this messes with -o -)
        "--no-simulate", // ensure video is downloaded regardless of printing
        "-f",
        "aac[abr>0]/bestaudio/best", // select best quality audio-only
        "-R",
        "infinite",        // infinite number of download retries
        "--no-playlist",   // only download the video if URL also has playlist info
        "--ignore-config", // disable all configuration files for a yt-dlp run
        "https://www.youtube.com/watch?v=MEg-oqI9qmw",
        "-o",
        "-", // stream data to stdout
    ];
    */

    let mut saved_spec: SignalSpec = SignalSpec { rate: 44100, channels: Channels::FRONT_LEFT | Channels::FRONT_RIGHT };

    let ytdl_args = [
        "-f",
        "m4a/bestaudio/best",
        "--throttled-rate",
        "100K",
        "https://www.youtube.com/watch?v=MEg-oqI9qmw",
        "-o",
        "-"
    ];

    // let handle = std::io::stdin();

    let mut yt = Command::new("yt-dlp")
        .args(&ytdl_args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Add a dummy source of the sake of the example.
    //let mut raw_source = ytdl("https://www.youtube.com/watch?v=MEg-oqI9qmw").await.unwrap();

    // Use the default options for format readers other than for gapless playback.
    let format_opts =
        FormatOptions { enable_gapless: true, ..Default::default() };

    // Use the default options for metadata readers.
    let metadata_opts: MetadataOptions = Default::default();


    let source = Box::new(ReadOnlySource::new(yt.stdout.take().unwrap())) as Box<dyn MediaSource>;

    let mss = MediaSourceStream::new(source, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate.
    let hint = Hint::new();

    let sampleData = Arc::new(Mutex::new(vec![] as Vec<f32>));

    // let mut databuff: Vec<f32> = Vec::new();

    match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
        Ok(probed) => {
            let mut reader = probed.format;
            let decoder_options = &DecoderOptions { verify: false, ..Default::default() };

            let track = reader.default_track().unwrap();

            let track_id = track.id;

            // let source_buffer = SamplesBuffer::new(2, track.codec_params.sample_rate.unwrap(), buffer);

            println!("{:?}", &track.codec_params);

            // Create a decoder for the track.
            let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, decoder_options).unwrap();

            // let mut audio_buffer: SampleBuffer<f32> = SampleBuffer::new(5000, saved_spec);

            let thread_arc = Arc::clone(&sampleData);

            let result = thread::spawn(move || {
                if let Ok(mut databuff) = thread_arc.lock() {
                    println!("Thread 1 acquired lock");
                    loop {
                        let packet = match reader.next_packet() {
                            Ok(packet) => packet,
                            Err(err) => break,
                        };
                
                        // If the packet does not belong to the selected track, skip over it.
                        if packet.track_id() != track_id {
                            continue;
                        }
        
                        
                
                        // Decode the packet into audio samples.
                        match decoder.decode(&packet) {
                            Ok(decoded) => {
                                let spec = *decoded.spec();
                                saved_spec = spec.clone();
                                let duration = decoded.capacity() as u64;
                                // println!("{:?}", spec);
                                let mut audio_buffer: SampleBuffer<f32> = SampleBuffer::new(duration, spec);
        
                                audio_buffer.copy_interleaved_ref(decoded);
        
                                // let source_buffer: SamplesBuffer<f32> = SamplesBuffer::new(saved_spec.channels.count() as u16, saved_spec.rate, audio_buffer.samples());
                                // sink.append(source_buffer);
    
                                let samples = audio_buffer.samples();
        
                                for samp in samples {
                                    databuff.push(samp.to_owned())
                                }
                                
                            },
                            Err(Error::DecodeError(err)) => print!("decode error: {}", err),
                            Err(err) => break,
                        }
                    };
                };
                
            });

            //let source_buffer: SamplesBuffer<f32> = SamplesBuffer::new(saved_spec.channels.count() as u16, saved_spec.rate, sampleData.lock().unwrap().as_slice());
            //result.join().expect("Could not fire handle");
            //sink.append(source_buffer);
            /* 
            let result: SymphResult<()> = loop {
                let packet = match reader.next_packet() {
                    Ok(packet) => packet,
                    Err(err) => break Err(err),
                };
        
                // If the packet does not belong to the selected track, skip over it.
                if packet.track_id() != track_id {
                    continue;
                }

                
        
                // Decode the packet into audio samples.
                match decoder.decode(&packet) {
                    Ok(decoded) => {
                        let spec = *decoded.spec();
                        saved_spec = spec.clone();
                        let duration = decoded.capacity() as u64;
                        // println!("{:?}", spec);
                        let mut audio_buffer: SampleBuffer<f32> = SampleBuffer::new(duration, spec);

                        audio_buffer.copy_interleaved_ref(decoded);

                        let source_buffer: SamplesBuffer<f32> = SamplesBuffer::new(saved_spec.channels.count() as u16, saved_spec.rate, audio_buffer.samples());
                        sink.append(source_buffer);
                        /* 
                        let samples = audio_buffer.samples();

                        for samp in samples {
                            databuff.push(samp.to_owned())
                        }
                        */
                        
                    },
                    Err(Error::DecodeError(err)) => print!("decode error: {}", err),
                    Err(err) => break Err(err),
                }
            };
            */
            result.join().expect("error aquiring lock")
        }
        Err(err) => {
            // The input was not supported by any format reader.
            print!("file not supported. reason? {}", err);
        }
    }

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    let source_buffer: SamplesBuffer<f32> = SamplesBuffer::new(saved_spec.channels.count() as u16, saved_spec.rate, sampleData.lock().unwrap().as_slice());
    sink.append(source_buffer);
    sink.sleep_until_end();

    Ok(())
}
