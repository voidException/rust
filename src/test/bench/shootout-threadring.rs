// The Computer Language Benchmarks Game
// http://benchmarksgame.alioth.debian.org/
//
// contributed by the Rust Project Developers

// Copyright (c) 2012-2014 The Rust Project Developers
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
// - Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in
//   the documentation and/or other materials provided with the
//   distribution.
//
// - Neither the name of "The Computer Language Benchmarks Game" nor
//   the name of "The Computer Language Shootout Benchmarks" nor the
//   names of its contributors may be used to endorse or promote
//   products derived from this software without specific prior
//   written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::Thread;

fn start(n_tasks: i32, token: i32) {
    let (tx, mut rx) = channel();
    tx.send(token).unwrap();
    let mut guards = Vec::with_capacity(n_tasks as usize);
    for i in 2 .. n_tasks + 1 {
        let (tx, next_rx) = channel();
        let cur_rx = std::mem::replace(&mut rx, next_rx);
        guards.push(Thread::scoped(move|| roundtrip(i, tx, cur_rx)));
    }
    let guard = Thread::scoped(move|| roundtrip(1, tx, rx));
}

fn roundtrip(id: i32, tx: Sender<i32>, rx: Receiver<i32>) {
    for token in rx.iter() {
        if token == 1 {
            println!("{}", id);
            break;
        }
        tx.send(token - 1).unwrap();
    }
}

fn main() {
    let args = std::os::args();
    let token = if std::os::getenv("RUST_BENCH").is_some() {
        2000000
    } else {
        args.get(1).and_then(|arg| arg.parse()).unwrap_or(1000)
    };
    let n_tasks = args.get(2)
                      .and_then(|arg| arg.parse())
                      .unwrap_or(503);

    start(n_tasks, token);
}
