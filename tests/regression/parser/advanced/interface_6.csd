// Interface test 6
collab Processable6 {
    slay process() drip
    slay getId() drip
}

squad Item6 {
    spill value drip
    
    slay process() drip {
        damn value * 6
    }
    
    slay getId() drip {
        damn 6
    }
}

sus item6 Item6 = Item6{value: 30}
sus processed drip = item6.process()
vibez.spill("Interface 6:", processed)
