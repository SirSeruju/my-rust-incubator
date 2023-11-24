use crossbeam_channel::bounded;
use packed_simd;
use rand::prelude::*;
use std::thread;

struct Producer;

impl Producer {
    fn start(sender: crossbeam_channel::Sender<[[u8; 64]; 64]>) -> thread::JoinHandle<()> {
        thread::spawn(move || loop {
            let matrix = [[0u8; 64]; 64];
            for mut sub_mat in matrix {
                rand::thread_rng().fill(&mut sub_mat);
            }

            sender.send(matrix).unwrap();
        })
    }
}

struct Consumer;

impl Consumer {
    fn start(receiver: crossbeam_channel::Receiver<[[u8; 64]; 64]>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for matrix in receiver.iter() {
                // Guaranteed not overflow for sum of u8x64x64
                let mut s = 0u32;
                for sub_mat in matrix {
                    // Packed u16x32 cause u16x64 not implemented
                    // u16 guaranteed not overflow for sum of u8x32
                    s += packed_simd::u16x32::from_slice_unaligned(
                        &sub_mat[..32]
                            .iter()
                            .map(|x| *x as u16)
                            .collect::<Vec<u16>>(),
                    )
                    .wrapping_sum() as u32;
                    s += packed_simd::u16x32::from_slice_unaligned(
                        &sub_mat[32..]
                            .iter()
                            .map(|x| *x as u16)
                            .collect::<Vec<u16>>(),
                    )
                    .wrapping_sum() as u32;
                }
                println!("{:?}", s);
            }
        })
    }
}

fn main() {
    let (s1, r1) = bounded::<[[u8; 64]; 64]>(0);
    let mut threads = Vec::new();
    threads.push(Producer::start(s1));
    threads.push(Consumer::start(r1.clone()));
    threads.push(Consumer::start(r1.clone()));

    for thread in threads {
        thread.join().unwrap();
    }
}
