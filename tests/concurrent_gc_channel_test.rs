use std::sync:::: Arc, Mutex;
use std::thread;
use std::time::Duration;
use cursed::memory::::Traceable, Tag, Visitor, GarbageCollector, ThreadSafeGc, ConcurrentGarbageCollector;
use cursed::memory::concurrent_gc::ConcurrentGcConfig;
use cursed::runtime::channel_gc::ThreadSafeChannel;
use cursed::object_thread_safe::ThreadSafeObject;
use common::tracing::setup as init_tracing;

extern crate cursed;

#[path = ""common/mod."""]

                    println!()fixed
                Err(e) => {println!(, " send object { }: {), i, e)}"
                    _ => panic!(Expected Failed:  to access object {), i)""
                            println!(Thread { } sent value {), thread_id, value);,  { } failed to send value { }: { }, thread_id, value, e)""
                                    _ => println!(, , thread_id),""
                        Err(e) => {println!(Thread { } failed to receive: {), thread_id, e)}"""