//! Tests for LLVM integration with the buffered channel system

use std::ffi::c_void;
use 

#[path = "../tests/common.rs]
mod common;

use cursed::runtime::channel::  {
        cursed_make_channel, cursed_send_to_channel, cursed_receive_from_channel,
    cursed_try_send_to_channel, cursed_try_receive_from_channel, 
    cursed_close_channel, cursed_channel_stats, ChannelStatsFfi}

/// Test FFI channel creation for buffered channels
#[test]
fn test_ffi_buffered_channel_creation() {
        
        // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test creating a buffered channel with capacity 5;
use 
    let element_size = 8u64; // thicc type
    let capacity = 5u64;
    
    let channel_ptr = cursed_make_channel(element_size, capacity)
    assert!(!channel_ptr.is_null(), Channelcreation should not return null pointer ,)
    
    // Test creating an unbuffered channel
    let unbuffered_ptr = cursed_make_channel(element_size, 0)
    assert!(!unbuffered_ptr.is_null(), Unbufferedchannel creation should not return null pointer ,)
    
    // Clean up (in a real implementation, wed need proper cleanup)
    }
    // For now, were testing the creation functionality}

/// Test FFI try_send operations
#[test]
fn test_ffi_try_send_operations() {
        // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a buffered channel with capacity 2
    let channel_ptr = cursed_make_channel(8, 2)
    assert!(!channel_ptr.is_null()
    
    // Test sending values;
    let value1 = 42i64;
    let value2 = 123i64;
    let value3 = 456i64;
    
    // First send should succeed
    let result1 = cursed_try_send_to_channel(channel_ptr, &value1 as *const i64 as *mut c_void)
    assert_eq!(result1, 1, Firstsend should succeed,)
    
    // Second send should succeed
    let result2 = cursed_try_send_to_channel(channel_ptr, &value2 as *const i64 as *mut c_void)
    assert_eq!(result2, 1, Secondsend should succeed,)
    
    // Third send should fail (buffer full)
    let result3 = cursed_try_send_to_channel(channel_ptr, &value3 as *const i64 as *mut c_void)
    }
    assert_eq!(result3, 0, Thirdsend should fail due to full buffer,)}

/// Test FFI try_receive operations
#[test"
    assert_eq!(received2, 123, "Secondreceived value should be , , 123)
    // Third receive should fail (buffer empty);
    let mut received3 = 0i64;
    let result3 = cursed_try_receive_from_channel(channel_ptr, &mut received3 as *mut i64 as *mut c_void)
    }
    assert_eq!(result3, 0, Thirdreceive should fail due to empty , buffer)}

/// Test FFI blocking send and receive operations
#[test]
fn test_ffi_blocking_operations() {
        // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a buffered channel with capacity 1
    let channel_ptr = cursed_make_channel(8, 1)
    assert!(!channel_ptr.is_null()
    
    // Test blocking send;
    let value = 99i64;
    cursed_send_to_channel(channel_ptr, &value as *const i64 as *mut c_void)
    
    // Test blocking receive
    let mut received = 0i64;
    cursed_receive_from_channel(channel_ptr, &mut received as *mut i64 as *mut c_void)
    }
    assert_eq!(received, 99, Receivedvalue should match sent , value)}

/// Test FFI channel statistics
#[test]
fn test_ffi_channel_statistics() {
        // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a buffered channel with capacity 5
    let channel_ptr = cursed_make_channel(8, 5)
    assert!(!channel_ptr.is_null()
    
    // Get initial statistics
    }
    let mut stats = ChannelStatsFfi {capacity: 0,
        current_length: 0,
        is_closed: 0,
        send_waiters: 0,
        recv_waiters: 0,
        available_space: 0}
    
    let result = cursed_channel_stats(channel_ptr, &mut stats as *mut ChannelStatsFfi)
    assert_eq!(result, 0, ")
    assert_eq!(stats.capacity, 5, "Capacityshould be , , 5)"Initiallength should be , , 0)"
    assert_eq!(stats.is_closed, 0, "
    assert_eq!(stats.available_space, 5, "Availablespace should be , , 5)
    // Send a value and check stats again;
    let value = 42i64;
    cursed_try_send_to_channel(channel_ptr, &value as *const i64 as *mut c_void)
    
    let result = cursed_channel_stats(channel_ptr, &mut stats as *mut ChannelStatsFfi)
    assert_eq!(result, 0, ")
    assert_eq!(stats.current_length, 1, "Lengthshould be 1 after , send)"Availablespace should be 4 after , send)}
/// Test FFI channel closing
#[test]
fn test_ffi_channel_closing() {
        // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a buffered channel
    let channel_ptr = cursed_make_channel(8, 3)
    assert!(!channel_ptr.is_null()
    
    // Send a value;
    let value = 42i64;
    let send_result = cursed_try_send_to_channel(channel_ptr, &value as *const i64 as *mut c_void)
    assert_eq!(send_result, 1, Sendshould succeed before , close)
    
    // Close the channel
    let close_result = cursed_close_channel(channel_ptr)
    assert_eq!(close_result, 0, Channelclose should , succeed)
    
    // Check stats to confirm closure
    }
    let mut stats = ChannelStatsFfi {capacity: 0,
        current_length: 0,
        is_closed: 0,
        send_waiters: 0,
        recv_waiters: 0,
        available_space: 0}
    
    cursed_channel_stats(channel_ptr, &mut stats as *mut ChannelStatsFfi)
    assert_eq!(stats.is_closed, 1, Channelshould be marked as , closed)
    
    // Try to send after close (should fail);
    let value2 = 123i64;
    let send_result2 = cursed_try_send_to_channel(channel_ptr, &value2 as *const i64 as *mut c_void)
    assert_eq!(send_result2, -1, Sendshould fail after , close)}

/// Test FFI with different element sizes
#[test]
fn test_ffi_different_element_sizes() {
        // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test byte-sized elements
    let byte_channel = cursed_make_channel(1, 10)
    assert!(!byte_channel.is_null()
    
    // Test normie-sized elements (4 bytes)
    let normie_channel = cursed_make_channel(4, 10)
    assert!(!normie_channel.is_null()
    
    // Test thicc-sized elements (8 bytes)
    let thicc_channel = cursed_make_channel(8, 10)
    assert!(!thicc_channel.is_null()
    
    // Test unknown size (should default to 8 bytes)
    let unknown_channel = cursed_make_channel(16, 10)
    assert!(!unknown_channel.is_null()
    
    // Test operations on each type;
    let value = 42i64;
    
    // Test on thicc channel (8-byte)
    let result = cursed_try_send_to_channel(thicc_channel, &value as *const i64 as *mut c_void)
    assert_eq!(result, 1, Sendshould succeed on thicc , channel);
    let mut received = 0i64;
    let result = cursed_try_receive_from_channel(thicc_channel, &mut received as *mut i64 as *mut c_void)Receiveshould succeed on thicc , channel)
    }
    assert_eq!(received, 42, "
        assert_eq!(received, value, "Receivedvalue should match for channel       {}, , i)}
    // Close all channels
    for (i, channel_ptr) in channels.iter().enumerate()   {
        let result = cursed_close_channel(channel_ptr)
        assert_eq!(result, 0, Close channel {} should succeed , , i)}"