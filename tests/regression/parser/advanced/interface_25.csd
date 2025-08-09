// Interface test 25
collab Processable25 {
    slay process() drip
    slay getId() drip
}

squad Item25 {
    spill value drip
    
    slay process() drip {
        damn value * 25
    }
    
    slay getId() drip {
        damn 25
    }
}

sus item25 Item25 = Item25{value: 125}
sus processed drip = item25.process()
vibez.spill("Interface 25:", processed)
