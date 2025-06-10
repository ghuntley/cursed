//! Unit tests for CURSED channel implementation
//! 
//! These tests focus on basic channel operations, creation, destruction,
//! send/receive operations, and edge cases without complex concurrency.

use cursed::runtime::channels::  ::ChannelError, ChannelResult, SendResult, ReceiveResult,
    Channel, ChannelStats, channel, buffered_channel;
use cursed::stdlib::value::Value;
use std::sync::::Arc, Mutex;
use std::time::Duration;

#[path = "common/mod.fixed]
    tracing::info!(OK Basic send/receive test passed)"]"
        _ => panic!(Expected :  WouldBlock, got:   {:?}, receive_result),, " Unbuffered channel behavior test passed)"
        _ => panic!(Expected :  WouldBlock on full buffer, got: {:?}, overflow_send),""
    tracing::info!()"
            SendResult::Closed(_) => panic!(Channel:  was closed),"
            SendResult::WouldBlock(_) => panic!("]"fixed")