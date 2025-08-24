yeet "websocketz"
yeet "stringz"
yeet "jsonz"

fr fr ========================================
fr fr CURSED WebSocket Integration Examples
fr fr Real-World Usage Scenarios & Patterns
fr fr ========================================

fr fr Chat Room Application Example
be_like ChatUser squad {
    spill user_id normie
    spill username tea
    spill connection_id normie
    spill joined_time normie
    spill last_active normie
    spill message_count normie
}

be_like ChatMessage squad {
    spill message_id normie
    spill user_id normie
    spill room_id tea
    spill content tea
    spill timestamp normie
    spill message_type tea  fr fr "text", "image", "file", "system"
}

slay chat_room_example() {
    vibez.spill("💬 Chat Room Application Example")
    vibez.spill("================================")
    
    fr fr Create WebSocket server for chat
    sus chat_server WebSocketConnection = ws_server_create(8080, "/chat")
    
    fr fr Create multiple chat rooms
    sus rooms [5]WebSocketRoom
    sus room_names [5]tea
    room_names[0] = "General Discussion"
    room_names[1] = "Tech Talk"
    room_names[2] = "Gaming Zone"
    room_names[3] = "Music & Arts"
    room_names[4] = "Random Topics"
    
    bestie i normie = 0; i < 5; i++ {
        sus room_id tea = "room_" + stringz.int_to_string(i)
        rooms[i] = ws_room_create(room_id, room_names[i])
    }
    
    fr fr Create chat users
    sus users [10]ChatUser
    sus usernames [10]tea
    usernames[0] = "Alice"
    usernames[1] = "Bob"
    usernames[2] = "Charlie"
    usernames[3] = "Diana"
    usernames[4] = "Eve"
    usernames[5] = "Frank"
    usernames[6] = "Grace"
    usernames[7] = "Henry"
    usernames[8] = "Ivy"
    usernames[9] = "Jack"
    
    bestie i normie = 0; i < 10; i++ {
        users[i].user_id = 3000 + i
        users[i].username = usernames[i]
        users[i].connection_id = 3000 + i
        users[i].joined_time = 1234567890 + i
        users[i].last_active = users[i].joined_time
        users[i].message_count = 0
    }
    
    fr fr Simulate users joining different rooms
    bestie user_idx normie = 0; user_idx < 10; user_idx++ {
        fr fr Each user joins 1-3 rooms
        sus room_count normie = (user_idx % 3) + 1
        bestie room_idx normie = 0; room_idx < room_count; room_idx++ {
            sus target_room normie = (user_idx + room_idx) % 5
            ws_room_join(&rooms[target_room], users[user_idx].connection_id)
        }
        
        fr fr Send join notification
        sus join_msg tea = users[user_idx].username + " joined the chat"
        bestie room_idx normie = 0; room_idx < 5; room_idx++ {
            fr fr Check if user is in this room
            bestie conn_idx normie = 0; conn_idx < rooms[room_idx].connection_count; conn_idx++ {
                lowkey rooms[room_idx].connections[conn_idx] == users[user_idx].connection_id {
                    ws_room_broadcast(rooms[room_idx], "📢 " + join_msg)
                    break
                }
            }
        }
    }
    
    fr fr Simulate chat conversation
    sus chat_messages [15]ChatMessage
    sus message_contents [15]tea
    message_contents[0] = "Hello everyone! 👋"
    message_contents[1] = "How's everyone doing today?"
    message_contents[2] = "Anyone working on interesting projects?"
    message_contents[3] = "Just finished a new WebSocket implementation!"
    message_contents[4] = "That sounds awesome! What language?"
    message_contents[5] = "CURSED programming language 🚀"
    message_contents[6] = "Never heard of it, but sounds cool!"
    message_contents[7] = "It's a new systems programming language"
    message_contents[8] = "Has anyone tried the new game that came out?"
    message_contents[9] = "Which game are you talking about?"
    message_contents[10] = "The one everyone's been talking about"
    message_contents[11] = "Oh that one! Yeah it's pretty good"
    message_contents[12] = "Anyone want to play multiplayer later?"
    message_contents[13] = "I'm up for it! What time?"
    message_contents[14] = "How about 8 PM? See you all there!"
    
    bestie msg_idx normie = 0; msg_idx < 15; msg_idx++ {
        chat_messages[msg_idx].message_id = 5000 + msg_idx
        chat_messages[msg_idx].user_id = 3000 + (msg_idx % 10)
        chat_messages[msg_idx].content = message_contents[msg_idx]
        chat_messages[msg_idx].timestamp = 1234567890 + (msg_idx * 60)
        chat_messages[msg_idx].message_type = "text"
        
        fr fr Assign to room based on message content
        lowkey stringz.contains(message_contents[msg_idx], "project") || 
             stringz.contains(message_contents[msg_idx], "CURSED") ||
             stringz.contains(message_contents[msg_idx], "language") {
            chat_messages[msg_idx].room_id = "room_1"  fr fr Tech Talk
        } elif stringz.contains(message_contents[msg_idx], "game") ||
              stringz.contains(message_contents[msg_idx], "multiplayer") {
            chat_messages[msg_idx].room_id = "room_2"  fr fr Gaming Zone
        } else {
            chat_messages[msg_idx].room_id = "room_0"  fr fr General Discussion
        }
        
        fr fr Find the room and broadcast message
        bestie room_idx normie = 0; room_idx < 5; room_idx++ {
            lowkey rooms[room_idx].room_id == chat_messages[msg_idx].room_id {
                sus sender_name tea = users[msg_idx % 10].username
                sus full_message tea = sender_name + ": " + message_contents[msg_idx]
                sus broadcast_count normie = ws_room_broadcast(rooms[room_idx], full_message)
                
                fr fr Update user message count
                users[msg_idx % 10].message_count++
                users[msg_idx % 10].last_active = chat_messages[msg_idx].timestamp
                break
            }
        }
    }
    
    fr fr Display chat room statistics
    vibez.spill("📊 Chat Room Statistics:")
    bestie room_idx normie = 0; room_idx < 5; room_idx++ {
        sus user_count normie = ws_room_get_connection_count(rooms[room_idx])
        vibez.spill("  " + room_names[room_idx] + ": " + 
                   stringz.int_to_string(user_count) + " users online")
    }
    
    fr fr Display user activity
    vibez.spill("👥 User Activity:")
    bestie user_idx normie = 0; user_idx < 10; user_idx++ {
        vibez.spill("  " + users[user_idx].username + ": " + 
                   stringz.int_to_string(users[user_idx].message_count) + " messages sent")
    }
    
    vibez.spill("✅ Chat Room Example completed!")
}

fr fr Real-Time Trading Platform Example
be_like TradingTick squad {
    spill symbol tea
    spill price normie
    spill volume normie
    spill timestamp normie
    spill change_percent normie
}

be_like TradingUser squad {
    spill user_id normie
    spill username tea
    spill subscriptions [20]tea  fr fr Subscribed symbols
    spill subscription_count normie
    spill connection_id normie
}

slay trading_platform_example() {
    vibez.spill("📈 Real-Time Trading Platform Example")
    vibez.spill("=====================================")
    
    fr fr Create WebSocket server for trading data
    sus trading_server WebSocketConnection = ws_server_create(8081, "/trading")
    
    fr fr Create rooms for different asset classes
    sus asset_rooms [4]WebSocketRoom
    asset_rooms[0] = ws_room_create("stocks", "Stock Market")
    asset_rooms[1] = ws_room_create("crypto", "Cryptocurrency")
    asset_rooms[2] = ws_room_create("forex", "Foreign Exchange")
    asset_rooms[3] = ws_room_create("commodities", "Commodities")
    
    fr fr Create trading users with different interests
    sus traders [8]TradingUser
    sus trader_names [8]tea
    trader_names[0] = "StockTrader"
    trader_names[1] = "CryptoInvestor"
    trader_names[2] = "ForexExpert"
    trader_names[3] = "CommodityBull"
    trader_names[4] = "DayTrader"
    trader_names[5] = "LongTermInvestor"
    trader_names[6] = "HedgeFundManager"
    trader_names[7] = "RetailInvestor"
    
    bestie i normie = 0; i < 8; i++ {
        traders[i].user_id = 4000 + i
        traders[i].username = trader_names[i]
        traders[i].connection_id = 4000 + i
        traders[i].subscription_count = 0
    }
    
    fr fr Set up subscriptions based on user interests
    fr fr StockTrader subscribes to stocks
    traders[0].subscriptions[0] = "AAPL"
    traders[0].subscriptions[1] = "GOOGL"
    traders[0].subscriptions[2] = "MSFT"
    traders[0].subscription_count = 3
    ws_room_join(&asset_rooms[0], traders[0].connection_id)
    
    fr fr CryptoInvestor subscribes to crypto
    traders[1].subscriptions[0] = "BTC"
    traders[1].subscriptions[1] = "ETH"
    traders[1].subscriptions[2] = "ADA"
    traders[1].subscription_count = 3
    ws_room_join(&asset_rooms[1], traders[1].connection_id)
    
    fr fr ForexExpert subscribes to forex
    traders[2].subscriptions[0] = "EUR/USD"
    traders[2].subscriptions[1] = "GBP/USD"
    traders[2].subscriptions[2] = "USD/JPY"
    traders[2].subscription_count = 3
    ws_room_join(&asset_rooms[2], traders[2].connection_id)
    
    fr fr DayTrader subscribes to everything
    traders[4].subscriptions[0] = "AAPL"
    traders[4].subscriptions[1] = "BTC"
    traders[4].subscriptions[2] = "EUR/USD"
    traders[4].subscriptions[3] = "GOLD"
    traders[4].subscription_count = 4
    bestie room_idx normie = 0; room_idx < 4; room_idx++ {
        ws_room_join(&asset_rooms[room_idx], traders[4].connection_id)
    }
    
    fr fr Generate sample market data
    sus ticks [12]TradingTick
    sus symbols [12]tea
    symbols[0] = "AAPL"
    symbols[1] = "GOOGL"
    symbols[2] = "MSFT"
    symbols[3] = "BTC"
    symbols[4] = "ETH"
    symbols[5] = "ADA"
    symbols[6] = "EUR/USD"
    symbols[7] = "GBP/USD"
    symbols[8] = "USD/JPY"
    symbols[9] = "GOLD"
    symbols[10] = "OIL"
    symbols[11] = "SILVER"
    
    sus prices [12]normie
    prices[0] = 15025  fr fr $150.25
    prices[1] = 285043 fr fr $2850.43
    prices[2] = 41289  fr fr $412.89
    prices[3] = 6850000 fr fr $68500.00
    prices[4] = 385062  fr fr $3850.62
    prices[5] = 125     fr fr $1.25
    prices[6] = 10843   fr fr 1.0843
    prices[7] = 12756   fr fr 1.2756
    prices[8] = 14892   fr fr 148.92
    prices[9] = 205634  fr fr $2056.34
    prices[10] = 8547   fr fr $85.47
    prices[11] = 2789   fr fr $27.89
    
    bestie tick_idx normie = 0; tick_idx < 12; tick_idx++ {
        ticks[tick_idx].symbol = symbols[tick_idx]
        ticks[tick_idx].price = prices[tick_idx]
        ticks[tick_idx].volume = 1000 + (tick_idx * 50)
        ticks[tick_idx].timestamp = 1234567890 + (tick_idx * 5)
        ticks[tick_idx].change_percent = (tick_idx % 3 == 0) ? 125 : -87  fr fr 1.25% or -0.87%
    }
    
    fr fr Simulate real-time market data broadcasting
    bestie tick_idx normie = 0; tick_idx < 12; tick_idx++ {
        sus tick TradingTick = ticks[tick_idx]
        
        fr fr Create market data message (JSON-like format)
        sus market_data tea = "{\"symbol\":\"" + tick.symbol + 
                             "\",\"price\":" + stringz.int_to_string(tick.price) +
                             ",\"volume\":" + stringz.int_to_string(tick.volume) +
                             ",\"change\":" + stringz.int_to_string(tick.change_percent) +
                             ",\"timestamp\":" + stringz.int_to_string(tick.timestamp) + "}"
        
        fr fr Determine which room to broadcast to
        sus room_index normie = 0
        lowkey stringz.contains(tick.symbol, "AAPL") || 
             stringz.contains(tick.symbol, "GOOGL") ||
             stringz.contains(tick.symbol, "MSFT") {
            room_index = 0  fr fr Stocks
        } elif stringz.contains(tick.symbol, "BTC") ||
              stringz.contains(tick.symbol, "ETH") ||
              stringz.contains(tick.symbol, "ADA") {
            room_index = 1  fr fr Crypto
        } elif stringz.contains(tick.symbol, "/") {
            room_index = 2  fr fr Forex
        } else {
            room_index = 3  fr fr Commodities
        }
        
        sus broadcast_count normie = ws_room_broadcast(asset_rooms[room_index], 
                                                      "📊 " + market_data)
        
        fr fr Simulate price alerts
        lowkey tick.change_percent > 100 {  fr fr > 1% change
            sus alert tea = "🚨 ALERT: " + tick.symbol + " up " + 
                           stringz.int_to_string(tick.change_percent) + "% at " + 
                           stringz.int_to_string(tick.price)
            ws_room_broadcast(asset_rooms[room_index], alert)
        }
    }
    
    fr fr Display trading platform statistics
    vibez.spill("📊 Trading Platform Statistics:")
    sus room_types [4]tea
    room_types[0] = "Stocks"
    room_types[1] = "Crypto"
    room_types[2] = "Forex"
    room_types[3] = "Commodities"
    
    bestie room_idx normie = 0; room_idx < 4; room_idx++ {
        sus subscriber_count normie = ws_room_get_connection_count(asset_rooms[room_idx])
        vibez.spill("  " + room_types[room_idx] + ": " + 
                   stringz.int_to_string(subscriber_count) + " subscribers")
    }
    
    vibez.spill("✅ Trading Platform Example completed!")
}

fr fr Game Multiplayer Lobby Example
be_like GamePlayer squad {
    spill player_id normie
    spill username tea
    spill level normie
    spill score normie
    spill status tea  fr fr "online", "in_game", "lobby", "offline"
    spill connection_id normie
}

be_like GameRoom squad {
    spill room_id tea
    spill game_mode tea
    spill max_players normie
    spill current_players normie
    spill status tea  fr fr "waiting", "starting", "in_progress", "finished"
}

slay multiplayer_game_example() {
    vibez.spill("🎮 Multiplayer Game Lobby Example")
    vibez.spill("=================================")
    
    fr fr Create WebSocket server for game
    sus game_server WebSocketConnection = ws_server_create(8082, "/game")
    
    fr fr Create game rooms for different modes
    sus game_rooms [3]WebSocketRoom
    game_rooms[0] = ws_room_create("battle_royale", "Battle Royale (100 players)")
    game_rooms[1] = ws_room_create("team_deathmatch", "Team Deathmatch (12 players)")
    game_rooms[2] = ws_room_create("co_op_campaign", "Co-op Campaign (4 players)")
    
    fr fr Create players with different skills
    sus players [15]GamePlayer
    sus player_names [15]tea
    player_names[0] = "ProGamer"
    player_names[1] = "NoobSlayer"
    player_names[2] = "CasualPlayer"
    player_names[3] = "EliteSharpshooter"
    player_names[4] = "TeamPlayer"
    player_names[5] = "SoloWolf"
    player_names[6] = "StrategyMaster"
    player_names[7] = "QuickScope"
    player_names[8] = "SupportMain"
    player_names[9] = "TankDestroyer"
    player_names[10] = "SpeedRunner"
    player_names[11] = "PuzzleSolver"
    player_names[12] = "LootGoblin"
    player_names[13] = "FriendlyFire"
    player_names[14] = "LastStand"
    
    bestie i normie = 0; i < 15; i++ {
        players[i].player_id = 5000 + i
        players[i].username = player_names[i]
        players[i].level = 10 + (i * 5)  fr fr Level 10-80
        players[i].score = 1000 + (i * 250)  fr fr Score 1000-4500
        players[i].status = "online"
        players[i].connection_id = 5000 + i
    }
    
    fr fr Players join different game modes based on preferences
    fr fr Battle Royale (high-level players)
    bestie i normie = 0; i < 8; i++ {
        lowkey players[i].level >= 40 {
            ws_room_join(&game_rooms[0], players[i].connection_id)
            players[i].status = "lobby"
        }
    }
    
    fr fr Team Deathmatch (medium-level players)
    bestie i normie = 3; i < 12; i++ {
        lowkey players[i].level >= 25 && players[i].level < 60 {
            ws_room_join(&game_rooms[1], players[i].connection_id)
            lowkey players[i].status == "online" {
                players[i].status = "lobby"
            }
        }
    }
    
    fr fr Co-op Campaign (any level, limited slots)
    bestie i normie = 10; i < 15; i++ {
        lowkey ws_room_get_connection_count(game_rooms[2]) < 4 {
            ws_room_join(&game_rooms[2], players[i].connection_id)
            players[i].status = "lobby"
        }
    }
    
    fr fr Simulate lobby chat and matchmaking
    sus lobby_messages [10]tea
    lobby_messages[0] = "Looking for a good team!"
    lobby_messages[1] = "Anyone want to play co-op?"
    lobby_messages[2] = "Battle Royale starting soon!"
    lobby_messages[3] = "Need one more player for team match"
    lobby_messages[4] = "Let's go for the win!"
    lobby_messages[5] = "First time playing this mode"
    lobby_messages[6] = "Who wants to team up?"
    lobby_messages[7] = "Ready to dominate!"
    lobby_messages[8] = "Good luck everyone!"
    lobby_messages[9] = "May the best team win!"
    
    bestie msg_idx normie = 0; msg_idx < 10; msg_idx++ {
        sus sender_idx normie = msg_idx % 15
        sus message tea = players[sender_idx].username + ": " + lobby_messages[msg_idx]
        
        fr fr Broadcast to appropriate room based on player's current room
        bestie room_idx normie = 0; room_idx < 3; room_idx++ {
            fr fr Check if player is in this room
            bestie conn_idx normie = 0; conn_idx < game_rooms[room_idx].connection_count; conn_idx++ {
                lowkey game_rooms[room_idx].connections[conn_idx] == players[sender_idx].connection_id {
                    ws_room_broadcast(game_rooms[room_idx], "💬 " + message)
                    break
                }
            }
        }
    }
    
    fr fr Simulate game start countdown
    sus countdown [3]tea
    countdown[0] = "Game starting in 3..."
    countdown[1] = "Game starting in 2..."
    countdown[2] = "Game starting in 1... GO!"
    
    bestie room_idx normie = 0; room_idx < 3; room_idx++ {
        lowkey ws_room_get_connection_count(game_rooms[room_idx]) > 0 {
            bestie countdown_idx normie = 0; countdown_idx < 3; countdown_idx++ {
                ws_room_broadcast(game_rooms[room_idx], "⏰ " + countdown[countdown_idx])
            }
            
            fr fr Mark players as in-game
            bestie conn_idx normie = 0; conn_idx < game_rooms[room_idx].connection_count; conn_idx++ {
                sus conn_id normie = game_rooms[room_idx].connections[conn_idx]
                bestie player_idx normie = 0; player_idx < 15; player_idx++ {
                    lowkey players[player_idx].connection_id == conn_id {
                        players[player_idx].status = "in_game"
                        break
                    }
                }
            }
        }
    }
    
    fr fr Display game statistics
    vibez.spill("🎮 Game Lobby Statistics:")
    sus mode_names [3]tea
    mode_names[0] = "Battle Royale"
    mode_names[1] = "Team Deathmatch"
    mode_names[2] = "Co-op Campaign"
    
    bestie room_idx normie = 0; room_idx < 3; room_idx++ {
        sus player_count normie = ws_room_get_connection_count(game_rooms[room_idx])
        vibez.spill("  " + mode_names[room_idx] + ": " + 
                   stringz.int_to_string(player_count) + " players")
    }
    
    fr fr Display player status
    vibez.spill("👤 Player Status:")
    sus status_counts [3]normie  fr fr online, lobby, in_game
    bestie player_idx normie = 0; player_idx < 15; player_idx++ {
        lowkey players[player_idx].status == "online" {
            status_counts[0]++
        } elif players[player_idx].status == "lobby" {
            status_counts[1]++
        } elif players[player_idx].status == "in_game" {
            status_counts[2]++
        }
    }
    
    vibez.spill("  Online: " + stringz.int_to_string(status_counts[0]) + " players")
    vibez.spill("  In Lobby: " + stringz.int_to_string(status_counts[1]) + " players")
    vibez.spill("  In Game: " + stringz.int_to_string(status_counts[2]) + " players")
    
    vibez.spill("✅ Multiplayer Game Example completed!")
}

fr fr IoT Device Monitoring Example
be_like IoTDevice squad {
    spill device_id tea
    spill device_type tea  fr fr "sensor", "actuator", "controller"
    spill location tea
    spill status tea  fr fr "online", "offline", "error", "maintenance"
    spill last_reading normie
    spill battery_level normie
    spill connection_id normie
}

be_like SensorReading squad {
    spill device_id tea
    spill sensor_type tea  fr fr "temperature", "humidity", "pressure", "light"
    spill value normie
    spill unit tea
    spill timestamp normie
    spill quality tea  fr fr "good", "warning", "error"
}

slay iot_monitoring_example() {
    vibez.spill("🔧 IoT Device Monitoring Example")
    vibez.spill("================================")
    
    fr fr Create WebSocket server for IoT monitoring
    sus iot_server WebSocketConnection = ws_server_create(8083, "/iot")
    
    fr fr Create rooms for different device types and locations
    sus monitoring_rooms [4]WebSocketRoom
    monitoring_rooms[0] = ws_room_create("temperature_sensors", "Temperature Monitoring")
    monitoring_rooms[1] = ws_room_create("security_devices", "Security System")
    monitoring_rooms[2] = ws_room_create("industrial_controllers", "Industrial Controllers")
    monitoring_rooms[3] = ws_room_create("environmental_sensors", "Environmental Monitoring")
    
    fr fr Create IoT devices
    sus devices [12]IoTDevice
    sus device_names [12]tea
    device_names[0] = "TEMP_SENSOR_01"
    device_names[1] = "TEMP_SENSOR_02" 
    device_names[2] = "HUMIDITY_SENSOR_01"
    device_names[3] = "PRESSURE_SENSOR_01"
    device_names[4] = "MOTION_DETECTOR_01"
    device_names[5] = "DOOR_SENSOR_01"
    device_names[6] = "CAMERA_01"
    device_names[7] = "PLC_CONTROLLER_01"
    device_names[8] = "VALVE_ACTUATOR_01"
    device_names[9] = "LIGHT_SENSOR_01"
    device_names[10] = "AIR_QUALITY_01"
    device_names[11] = "SMOKE_DETECTOR_01"
    
    sus locations [12]tea
    locations[0] = "Server Room A"
    locations[1] = "Server Room B"
    locations[2] = "Warehouse Floor 1"
    locations[3] = "Production Line 1"
    locations[4] = "Main Entrance"
    locations[5] = "Emergency Exit"
    locations[6] = "Parking Garage"
    locations[7] = "Control Room"
    locations[8] = "Chemical Storage"
    locations[9] = "Office Area"
    locations[10] = "HVAC System"
    locations[11] = "Kitchen Area"
    
    bestie i normie = 0; i < 12; i++ {
        devices[i].device_id = device_names[i]
        devices[i].location = locations[i]
        devices[i].status = "online"
        devices[i].last_reading = 1234567890 + (i * 30)
        devices[i].battery_level = 85 + (i % 15)  fr fr 85-99%
        devices[i].connection_id = 6000 + i
        
        fr fr Set device type based on name
        lowkey stringz.contains(device_names[i], "TEMP") || 
             stringz.contains(device_names[i], "HUMIDITY") ||
             stringz.contains(device_names[i], "PRESSURE") ||
             stringz.contains(device_names[i], "LIGHT") ||
             stringz.contains(device_names[i], "AIR") ||
             stringz.contains(device_names[i], "SMOKE") {
            devices[i].device_type = "sensor"
        } elif stringz.contains(device_names[i], "VALVE") {
            devices[i].device_type = "actuator"
        } else {
            devices[i].device_type = "controller"
        }
    }
    
    fr fr Assign devices to monitoring rooms
    bestie i normie = 0; i < 12; i++ {
        lowkey stringz.contains(devices[i].device_id, "TEMP") {
            ws_room_join(&monitoring_rooms[0], devices[i].connection_id)
        } elif stringz.contains(devices[i].device_id, "MOTION") ||
              stringz.contains(devices[i].device_id, "DOOR") ||
              stringz.contains(devices[i].device_id, "CAMERA") ||
              stringz.contains(devices[i].device_id, "SMOKE") {
            ws_room_join(&monitoring_rooms[1], devices[i].connection_id)
        } elif stringz.contains(devices[i].device_id, "PLC") ||
              stringz.contains(devices[i].device_id, "VALVE") {
            ws_room_join(&monitoring_rooms[2], devices[i].connection_id)
        } else {
            ws_room_join(&monitoring_rooms[3], devices[i].connection_id)
        }
    }
    
    fr fr Generate sensor readings
    sus readings [20]SensorReading
    sus reading_values [20]normie
    reading_values[0] = 2245   fr fr 22.45°C
    reading_values[1] = 2387   fr fr 23.87°C
    reading_values[2] = 4567   fr fr 45.67% humidity
    reading_values[3] = 101325 fr fr 101325 Pa pressure
    reading_values[4] = 1      fr fr Motion detected
    reading_values[5] = 0      fr fr Door closed
    reading_values[6] = 1      fr fr Camera active
    reading_values[7] = 750    fr fr 750 lux
    reading_values[8] = 42     fr fr Air quality index
    reading_values[9] = 0      fr fr No smoke
    reading_values[10] = 2156  fr fr 21.56°C
    reading_values[11] = 2401  fr fr 24.01°C
    reading_values[12] = 4789  fr fr 47.89% humidity
    reading_values[13] = 101298 fr fr 101298 Pa pressure
    reading_values[14] = 0     fr fr No motion
    reading_values[15] = 1     fr fr Door open
    reading_values[16] = 685   fr fr 685 lux
    reading_values[17] = 38    fr fr Air quality index
    reading_values[18] = 2678  fr fr 26.78°C (warning)
    reading_values[19] = 8901  fr fr 89.01% humidity (high)
    
    bestie reading_idx normie = 0; reading_idx < 20; reading_idx++ {
        sus device_idx normie = reading_idx % 12
        readings[reading_idx].device_id = devices[device_idx].device_id
        readings[reading_idx].value = reading_values[reading_idx]
        readings[reading_idx].timestamp = 1234567890 + (reading_idx * 60)
        
        fr fr Set sensor type and unit based on device
        lowkey stringz.contains(devices[device_idx].device_id, "TEMP") {
            readings[reading_idx].sensor_type = "temperature"
            readings[reading_idx].unit = "°C"
            fr fr Check for warning conditions
            lowkey readings[reading_idx].value > 2500 {  fr fr > 25°C
                readings[reading_idx].quality = "warning"
                devices[device_idx].status = "warning"
            } else {
                readings[reading_idx].quality = "good"
            }
        } elif stringz.contains(devices[device_idx].device_id, "HUMIDITY") {
            readings[reading_idx].sensor_type = "humidity"
            readings[reading_idx].unit = "%"
            lowkey readings[reading_idx].value > 8000 {  fr fr > 80%
                readings[reading_idx].quality = "warning"
            } else {
                readings[reading_idx].quality = "good"
            }
        } elif stringz.contains(devices[device_idx].device_id, "PRESSURE") {
            readings[reading_idx].sensor_type = "pressure"
            readings[reading_idx].unit = "Pa"
            readings[reading_idx].quality = "good"
        } elif stringz.contains(devices[device_idx].device_id, "LIGHT") {
            readings[reading_idx].sensor_type = "light"
            readings[reading_idx].unit = "lux"
            readings[reading_idx].quality = "good"
        } else {
            readings[reading_idx].sensor_type = "binary"
            readings[reading_idx].unit = "state"
            readings[reading_idx].quality = "good"
        }
    }
    
    fr fr Broadcast sensor readings to appropriate rooms
    bestie reading_idx normie = 0; reading_idx < 20; reading_idx++ {
        sus reading SensorReading = readings[reading_idx]
        
        fr fr Create sensor data message
        sus sensor_data tea = "{\"device\":\"" + reading.device_id + 
                             "\",\"type\":\"" + reading.sensor_type +
                             "\",\"value\":" + stringz.int_to_string(reading.value) +
                             ",\"unit\":\"" + reading.unit +
                             "\",\"quality\":\"" + reading.quality +
                             "\",\"timestamp\":" + stringz.int_to_string(reading.timestamp) + "}"
        
        fr fr Find device's room and broadcast
        sus device_idx normie = reading_idx % 12
        bestie room_idx normie = 0; room_idx < 4; room_idx++ {
            fr fr Check if device is in this room
            bestie conn_idx normie = 0; conn_idx < monitoring_rooms[room_idx].connection_count; conn_idx++ {
                lowkey monitoring_rooms[room_idx].connections[conn_idx] == devices[device_idx].connection_id {
                    sus message tea = "📊 " + sensor_data
                    lowkey reading.quality == "warning" {
                        message = "⚠️ " + sensor_data
                    }
                    ws_room_broadcast(monitoring_rooms[room_idx], message)
                    break
                }
            }
        }
        
        fr fr Send alerts for critical conditions
        lowkey reading.quality == "warning" {
            sus alert_msg tea = "🚨 ALERT: " + reading.device_id + " - " + 
                               reading.sensor_type + " reading " + 
                               stringz.int_to_string(reading.value) + " " + 
                               reading.unit + " exceeds normal range"
            
            fr fr Broadcast alert to all relevant rooms
            bestie room_idx normie = 0; room_idx < 4; room_idx++ {
                bestie conn_idx normie = 0; conn_idx < monitoring_rooms[room_idx].connection_count; conn_idx++ {
                    lowkey monitoring_rooms[room_idx].connections[conn_idx] == devices[device_idx].connection_id {
                        ws_room_broadcast(monitoring_rooms[room_idx], alert_msg)
                        break
                    }
                }
            }
        }
    }
    
    fr fr Display monitoring statistics
    vibez.spill("🔧 IoT Monitoring Statistics:")
    sus room_types [4]tea
    room_types[0] = "Temperature Sensors"
    room_types[1] = "Security Devices"
    room_types[2] = "Industrial Controllers"
    room_types[3] = "Environmental Sensors"
    
    bestie room_idx normie = 0; room_idx < 4; room_idx++ {
        sus device_count normie = ws_room_get_connection_count(monitoring_rooms[room_idx])
        vibez.spill("  " + room_types[room_idx] + ": " + 
                   stringz.int_to_string(device_count) + " devices")
    }
    
    fr fr Display device status summary
    sus status_counts [4]normie  fr fr online, warning, error, offline
    bestie device_idx normie = 0; device_idx < 12; device_idx++ {
        lowkey devices[device_idx].status == "online" {
            status_counts[0]++
        } elif devices[device_idx].status == "warning" {
            status_counts[1]++
        } elif devices[device_idx].status == "error" {
            status_counts[2]++
        } else {
            status_counts[3]++
        }
    }
    
    vibez.spill("📈 Device Status Summary:")
    vibez.spill("  Online: " + stringz.int_to_string(status_counts[0]) + " devices")
    vibez.spill("  Warning: " + stringz.int_to_string(status_counts[1]) + " devices")
    vibez.spill("  Error: " + stringz.int_to_string(status_counts[2]) + " devices")
    vibez.spill("  Offline: " + stringz.int_to_string(status_counts[3]) + " devices")
    
    vibez.spill("✅ IoT Monitoring Example completed!")
}

fr fr =============================================================================
fr fr INTEGRATION EXAMPLES EXECUTION
fr fr =============================================================================

slay run_all_integration_examples() {
    vibez.spill("🌐 Running WebSocket Integration Examples")
    vibez.spill("=========================================")
    
    chat_room_example()
    vibez.spill("")
    
    trading_platform_example()
    vibez.spill("")
    
    multiplayer_game_example()
    vibez.spill("")
    
    iot_monitoring_example()
    vibez.spill("")
    
    vibez.spill("✅ All WebSocket Integration Examples completed!")
    vibez.spill("")
    vibez.spill("📋 Summary:")
    vibez.spill("  • Chat Room: Multi-room messaging with user management")
    vibez.spill("  • Trading Platform: Real-time market data broadcasting")
    vibez.spill("  • Game Lobby: Multiplayer matchmaking and communication")
    vibez.spill("  • IoT Monitoring: Device status and sensor data streaming")
    vibez.spill("")
    vibez.spill("These examples demonstrate production-ready WebSocket usage patterns")
    vibez.spill("for common real-world applications and use cases.")
}

fr fr Run all integration examples
run_all_integration_examples()
