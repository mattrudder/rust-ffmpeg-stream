use ffmpeg4_ffi::sys::{
    av_dump_format, av_free_packet, av_gettime, av_interleaved_write_frame, av_read_frame,
    av_register_all, av_rescale_q, av_rescale_q_rnd, av_usleep, av_version_info, av_write_trailer,
    avcodec_copy_context, avformat_alloc_output_context2, avformat_close_input,
    avformat_find_stream_info, avformat_free_context, avformat_network_init, avformat_new_stream,
    avformat_open_input, avformat_write_header, avio_close, avio_open, AVFormatContext,
    AVMediaType_AVMEDIA_TYPE_VIDEO, AVPacket, AVRational, AVRounding_AV_ROUND_NEAR_INF,
    AVRounding_AV_ROUND_PASS_MINMAX, AVFMT_GLOBALHEADER, AVFMT_NOFILE, AVIO_FLAG_WRITE,
    AV_CODEC_FLAG_GLOBAL_HEADER, AV_TIME_BASE,
};
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

fn av_q2d(a: AVRational) -> f64 {
    a.num as f64 / a.den as f64
}

#[allow(overflowing_literals)]
fn av_nopts_value() -> i64 {
    0x8000000000000000 as i64
}

unsafe fn cleanup(ifmt_context: *mut *mut AVFormatContext, ofmt_context: *mut AVFormatContext) {
    avformat_close_input(ifmt_context);
    if ofmt_context != null_mut() && ((*(*ofmt_context).oformat).flags & AVFMT_NOFILE as i32) == 0 {
        avio_close((*ofmt_context).pb);
    }

    avformat_free_context(ofmt_context);
}

fn main() {
    unsafe {
        av_register_all();
        avformat_network_init();

        let p = av_version_info();
        let pstr = CStr::from_ptr(p);
        println!("Hi from ffmpeg: {}", pstr.to_string_lossy());

        let mut video_index = 0;
        let mut ifmt_context = null_mut();
        let mut ofmt_context = null_mut();
        let in_path = CString::new("C:\\Users\\rudder\\Downloads\\trailer_1080p.mov").unwrap();
        if avformat_open_input(&mut ifmt_context, in_path.as_ptr(), null_mut(), null_mut()) < 0 {
            println!("couldn't open input file");
            cleanup(&mut ifmt_context, ofmt_context);
            return;
        }

        if avformat_find_stream_info(ifmt_context, null_mut()) < 0 {
            println!("couldn't find stream info");
            cleanup(&mut ifmt_context, ofmt_context);
            return;
        }

        let in_streams = std::slice::from_raw_parts(
            (*ifmt_context).streams,
            (*ifmt_context).nb_streams as usize,
        );
        for (i, stream) in in_streams.iter().enumerate() {
            if (*(*(*stream)).codec).codec_type == AVMediaType_AVMEDIA_TYPE_VIDEO {
                video_index = i;
                break;
            }
        }

        av_dump_format(ifmt_context, 0, in_path.as_ptr(), 0);

        let out_path =
            CString::new(std::env::var("STREAM_KEY").expect("Must have STREAM_KEY set")).unwrap();
        let fmt = CString::new("flv").unwrap();
        avformat_alloc_output_context2(
            &mut ofmt_context,
            null_mut(),
            fmt.as_ptr(),
            out_path.as_ptr(),
        );

        if ofmt_context == null_mut() {
            println!("unable to create output");
            cleanup(&mut ifmt_context, ofmt_context);
            return;
        }

        let ofmt = (*ofmt_context).oformat;
        for in_stream in in_streams {
            let out_stream = avformat_new_stream(ofmt_context, (*(*(*in_stream)).codec).codec);
            if out_stream == null_mut() {
                println!("unable to create out stream");
                cleanup(&mut ifmt_context, ofmt_context);
                return;
            }

            let ret = avcodec_copy_context((*out_stream).codec, (*(*in_stream)).codec);
            if ret < 0 {
                println!("unable to copy contexts");
                cleanup(&mut ifmt_context, ofmt_context);
                return;
            }

            (*(*out_stream).codec).codec_tag = 0;
            if ((*(*ofmt_context).oformat).flags & AVFMT_GLOBALHEADER as i32) != 0 {
                (*(*out_stream).codec).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
            }
        }

        av_dump_format(ofmt_context, 0, out_path.as_ptr(), 1);
        if ((*ofmt).flags & AVFMT_NOFILE as i32) == 0 {
            let ret = avio_open(
                &mut (*ofmt_context).pb,
                out_path.as_ptr(),
                AVIO_FLAG_WRITE as i32,
            );
            if ret < 0 {
                println!("unable to open out stream");
                cleanup(&mut ifmt_context, ofmt_context);
                return;
            }
        }

        let ret = avformat_write_header(ofmt_context, null_mut());
        if ret < 0 {
            println!("unable to write header");
            cleanup(&mut ifmt_context, ofmt_context);
            return;
        }

        let out_streams = std::slice::from_raw_parts(
            (*ofmt_context).streams,
            (*ofmt_context).nb_streams as usize,
        );
        let mut pkt: AVPacket = AVPacket {
            buf: null_mut(),
            pts: 0,
            dts: 0,
            data: null_mut(),
            size: 0,
            stream_index: 0,
            flags: 0,
            side_data: null_mut(),
            side_data_elems: 0,
            duration: 0,
            pos: 0,
            convergence_duration: 0,
        };
        let mut frame_index = 0;
        let start_time = av_gettime();
        loop {
            let ret = av_read_frame(ifmt_context, &mut pkt);
            if ret < 0 {
                break;
            }

            let video_stream = in_streams[video_index];
            if pkt.pts == av_nopts_value() {
                let time_base = (*video_stream).time_base;
                let calc_duration = AV_TIME_BASE as f64 / av_q2d((*video_stream).r_frame_rate);
                pkt.pts = (frame_index as f64 * calc_duration / av_q2d(time_base)
                    * AV_TIME_BASE as f64) as i64;
                pkt.dts = pkt.pts;
                pkt.duration = (calc_duration / (av_q2d(time_base) * AV_TIME_BASE as f64)) as i64;
            }

            if pkt.stream_index == video_index as i32 {
                let time_base = (*video_stream).time_base;
                let time_base_q = AVRational {
                    num: 1,
                    den: AV_TIME_BASE as i32,
                };
                let pts_time = av_rescale_q(pkt.dts, time_base, time_base_q);
                let now_time = av_gettime() - start_time;
                if pts_time > now_time {
                    av_usleep((pts_time - now_time) as u32);
                }
            }

            let in_stream = in_streams[pkt.stream_index as usize];
            let out_stream = out_streams[pkt.stream_index as usize];
            pkt.pts = av_rescale_q_rnd(
                pkt.pts,
                (*in_stream).time_base,
                (*out_stream).time_base,
                AVRounding_AV_ROUND_NEAR_INF | AVRounding_AV_ROUND_PASS_MINMAX,
            );
            pkt.dts = av_rescale_q_rnd(
                pkt.dts,
                (*in_stream).time_base,
                (*out_stream).time_base,
                AVRounding_AV_ROUND_NEAR_INF | AVRounding_AV_ROUND_PASS_MINMAX,
            );
            pkt.duration = av_rescale_q(
                pkt.duration,
                (*in_stream).time_base,
                (*out_stream).time_base,
            );
            pkt.pos = -1;

            if pkt.stream_index == video_index as i32 {
                println!("sent {} frames to output", frame_index);
                frame_index += 1;
            }

            let ret = av_interleaved_write_frame(ofmt_context, &mut pkt);
            if ret < 0 {
                println!("error muxing packet");
                break;
            }

            av_free_packet(&mut pkt);
        }

        av_write_trailer(ofmt_context);
        cleanup(&mut ifmt_context, ofmt_context);
    }
}
