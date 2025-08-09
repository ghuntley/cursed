// Interface test 8
collab Processable8 {
    slay process() drip
    slay getId() drip
}

squad Item8 {
    spill value drip
    
    slay process() drip {
        damn value * 8
    }
    
    slay getId() drip {
        damn 8
    }
}

sus item8 Item8 = Item8{value: 40}
sus processed drip = item8.process()
vibez.spill("Interface 8:", processed)
