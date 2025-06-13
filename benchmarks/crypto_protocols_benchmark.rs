/// Cryptographic Protocols Performance Benchmarks
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cursed::stdlib::packages::crypto_protocols::*;
use std::time::Duration;

fn benchmark_key_exchange(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_exchange");
    
    let manager = KeyExchangeManager::new().unwrap();
    let protocols = vec![
        KeyExchangeProtocol::X25519,
        KeyExchangeProtocol::ECDH,
        KeyExchangeProtocol::DiffieHellman,
        KeyExchangeProtocol::Kyber1024,
    ];
    
    for protocol in protocols {
        group.bench_with_input(
            BenchmarkId::new("initiate", format!("{:?}", protocol)),
            &protocol,
            |b, protocol| {
                b.iter(|| {
                    manager.initiate_exchange(protocol.clone()).unwrap()
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("complete", format!("{:?}", protocol)),
            &protocol,
            |b, protocol| {
                let result = manager.initiate_exchange(protocol.clone()).unwrap();
                let peer_key = vec![0x42; 32];
                b.iter(|| {
                    manager.complete_exchange(&result.session_id, peer_key.clone()).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_ecdh_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecdh");
    
    let manager = EcdhManager::new().unwrap();
    let curves = vec![
        EcdhCurve::P256,
        EcdhCurve::P384,
        EcdhCurve::X25519,
        EcdhCurve::Secp256k1,
    ];
    
    for curve in curves {
        group.bench_with_input(
            BenchmarkId::new("keygen", format!("{:?}", curve)),
            &curve,
            |b, curve| {
                b.iter(|| {
                    manager.generate_keypair(curve.clone()).unwrap()
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("shared_secret", format!("{:?}", curve)),
            &curve,
            |b, curve| {
                let keypair1 = manager.generate_keypair(curve.clone()).unwrap();
                let keypair2 = manager.generate_keypair(curve.clone()).unwrap();
                b.iter(|| {
                    manager.compute_shared_secret(&keypair1, &keypair2.public_key).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_diffie_hellman(c: &mut Criterion) {
    let mut group = c.benchmark_group("diffie_hellman");
    
    let manager = DiffieHellmanManager::new().unwrap();
    let groups = vec![
        DhGroup::Group14, // 2048-bit
        DhGroup::Group15, // 3072-bit
        DhGroup::Group16, // 4096-bit
    ];
    
    for dh_group in groups {
        group.bench_with_input(
            BenchmarkId::new("keygen", format!("{:?}", dh_group)),
            &dh_group,
            |b, group| {
                b.iter(|| {
                    manager.generate_keypair(group.clone()).unwrap()
                });
            },
        );
        
        // Note: Only benchmark smaller groups for performance
        if matches!(dh_group, DhGroup::Group14) {
            group.bench_with_input(
                BenchmarkId::new("shared_secret", format!("{:?}", dh_group)),
                &dh_group,
                |b, group| {
                    let keypair1 = manager.generate_keypair(group.clone()).unwrap();
                    let keypair2 = manager.generate_keypair(group.clone()).unwrap();
                    b.iter(|| {
                        manager.compute_shared_secret(&keypair1, &keypair2.public_key).unwrap()
                    });
                },
            );
        }
    }
    
    group.finish();
}

fn benchmark_authentication(c: &mut Criterion) {
    let mut group = c.benchmark_group("authentication");
    
    let manager = AuthenticationManager::new().unwrap();
    let secret = b"test_secret_key_1234567890123456";
    
    group.bench_function("totp_generation", |b| {
        b.iter(|| {
            manager.generate_totp(secret, Some(30)).unwrap()
        });
    });
    
    group.bench_function("hotp_generation", |b| {
        b.iter(|| {
            manager.generate_hotp(secret, 12345).unwrap()
        });
    });
    
    group.bench_function("totp_verification", |b| {
        let totp = manager.generate_totp(secret, Some(30)).unwrap();
        b.iter(|| {
            manager.verify_totp(secret, &totp, Some(1)).unwrap()
        });
    });
    
    group.bench_function("auth_session_start", |b| {
        b.iter(|| {
            manager.start_authentication("test_user", None).unwrap()
        });
    });
    
    group.finish();
}

fn benchmark_signal_protocol(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_protocol");
    
    let manager = SignalProtocolManager::new().unwrap();
    
    group.bench_function("key_bundle_generation", |b| {
        b.iter(|| {
            manager.generate_key_bundle(5).unwrap()
        });
    });
    
    group.bench_function("session_start", |b| {
        let key_bundle = manager.generate_key_bundle(3).unwrap();
        b.iter(|| {
            manager.start_session("test_user", key_bundle.clone()).unwrap()
        });
    });
    
    group.bench_function("message_encryption", |b| {
        let key_bundle = manager.generate_key_bundle(3).unwrap();
        let session_id = manager.start_session("test_user", key_bundle).unwrap();
        let message = b"Hello, this is a test message for benchmarking!";
        
        b.iter(|| {
            manager.encrypt_message(&session_id, message).unwrap()
        });
    });
    
    group.bench_function("message_decryption", |b| {
        let key_bundle = manager.generate_key_bundle(3).unwrap();
        let session_id = manager.start_session("test_user", key_bundle).unwrap();
        let message = b"Hello, this is a test message for benchmarking!";
        let encrypted = manager.encrypt_message(&session_id, message).unwrap();
        
        b.iter(|| {
            manager.decrypt_message(encrypted.clone()).unwrap()
        });
    });
    
    group.finish();
}

fn benchmark_tls_handshake(c: &mut Criterion) {
    let mut group = c.benchmark_group("tls_handshake");
    
    let manager = TlsHandshakeManager::new().unwrap();
    
    group.bench_function("client_handshake_start", |b| {
        b.iter(|| {
            manager.start_client_handshake(None).unwrap()
        });
    });
    
    group.bench_function("server_handshake_start", |b| {
        b.iter(|| {
            manager.start_server_handshake(None).unwrap()
        });
    });
    
    group.bench_function("handshake_message_processing", |b| {
        let (session_id, client_hello) = manager.start_client_handshake(None).unwrap();
        
        b.iter(|| {
            // Simulate processing (this is a simplified benchmark)
            manager.get_session(&session_id).unwrap()
        });
    });
    
    group.finish();
}

fn benchmark_session_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_management");
    
    let manager = SessionManager::new().unwrap();
    
    group.bench_function("session_creation", |b| {
        b.iter(|| {
            manager.create_session("test_peer", None).unwrap()
        });
    });
    
    group.bench_function("session_activation", |b| {
        let session_id = manager.create_session("test_peer", None).unwrap();
        b.iter(|| {
            manager.activate_session(&session_id).unwrap()
        });
    });
    
    group.bench_function("activity_recording", |b| {
        let session_id = manager.create_session("test_peer", None).unwrap();
        manager.activate_session(&session_id).unwrap();
        
        b.iter(|| {
            manager.record_activity(&session_id, 1024, 512).unwrap()
        });
    });
    
    group.bench_function("session_ticket_creation", |b| {
        let session_id = manager.create_session("test_peer", None).unwrap();
        manager.activate_session(&session_id).unwrap();
        
        b.iter(|| {
            manager.create_session_ticket(&session_id).unwrap()
        });
    });
    
    group.bench_function("session_resumption", |b| {
        let session_id = manager.create_session("test_peer", None).unwrap();
        manager.activate_session(&session_id).unwrap();
        let ticket = manager.create_session_ticket(&session_id).unwrap();
        
        b.iter(|| {
            manager.resume_session(&ticket.ticket_id, "test_peer").unwrap()
        });
    });
    
    group.finish();
}

fn benchmark_secure_channels(c: &mut Criterion) {
    let mut group = c.benchmark_group("secure_channels");
    
    let manager = SecureChannelManager::new().unwrap();
    
    group.bench_function("channel_creation", |b| {
        b.iter(|| {
            manager.create_channel(None).unwrap()
        });
    });
    
    group.bench_function("handshake_step", |b| {
        let channel_id = manager.create_channel(None).unwrap();
        
        b.iter(|| {
            manager.handshake_step(&channel_id, None).unwrap()
        });
    });
    
    group.finish();
}

fn benchmark_protocol_suite(c: &mut Criterion) {
    let mut group = c.benchmark_group("protocol_suite");
    
    group.bench_function("suite_creation", |b| {
        b.iter(|| {
            create_protocol_suite().unwrap()
        });
    });
    
    group.bench_function("complete_key_exchange_flow", |b| {
        let suite = create_protocol_suite().unwrap();
        
        b.iter(|| {
            let result = suite.key_exchange.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
            let peer_key = vec![0x42; 32];
            suite.key_exchange.complete_exchange(&result.session_id, peer_key).unwrap()
        });
    });
    
    group.bench_function("complete_authentication_flow", |b| {
        let suite = create_protocol_suite().unwrap();
        
        b.iter(|| {
            let result = suite.authentication.start_authentication("test_user", None).unwrap();
            if let Some(challenge) = result.next_challenge {
                let response = b"test_password";
                suite.authentication.respond_to_challenge(&challenge.challenge_id, response).unwrap()
            }
        });
    });
    
    group.finish();
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("parallel_key_exchanges", |b| {
        use std::sync::Arc;
        use std::thread;
        
        let manager = Arc::new(KeyExchangeManager::new().unwrap());
        
        b.iter(|| {
            let mut handles = vec![];
            
            for i in 0..4 {
                let mgr = Arc::clone(&manager);
                let handle = thread::spawn(move || {
                    let result = mgr.initiate_exchange(KeyExchangeProtocol::X25519).unwrap();
                    let peer_key = vec![0x42 + i as u8; 32];
                    mgr.complete_exchange(&result.session_id, peer_key).unwrap()
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    
    group.bench_function("parallel_ecdh_operations", |b| {
        use std::sync::Arc;
        use std::thread;
        
        let manager = Arc::new(EcdhManager::new().unwrap());
        
        b.iter(|| {
            let mut handles = vec![];
            
            for _ in 0..4 {
                let mgr = Arc::clone(&manager);
                let handle = thread::spawn(move || {
                    let keypair1 = mgr.generate_keypair(EcdhCurve::X25519).unwrap();
                    let keypair2 = mgr.generate_keypair(EcdhCurve::X25519).unwrap();
                    mgr.compute_shared_secret(&keypair1, &keypair2.public_key).unwrap()
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_key_exchange,
    benchmark_ecdh_operations,
    benchmark_diffie_hellman,
    benchmark_authentication,
    benchmark_signal_protocol,
    benchmark_tls_handshake,
    benchmark_session_management,
    benchmark_secure_channels,
    benchmark_protocol_suite,
    benchmark_concurrent_operations,
);

criterion_main!(benches);
