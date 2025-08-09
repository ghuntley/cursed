// Interface test 21
collab Processable21 {
    slay process() drip
    slay getId() drip
}

squad Item21 {
    spill value drip
    
    slay process() drip {
        damn value * 21
    }
    
    slay getId() drip {
        damn 21
    }
}

sus item21 Item21 = Item21{value: 105}
sus processed drip = item21.process()
vibez.spill("Interface 21:", processed)
