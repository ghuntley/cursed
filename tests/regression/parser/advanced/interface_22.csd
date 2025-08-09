// Interface test 22
collab Processable22 {
    slay process() drip
    slay getId() drip
}

squad Item22 {
    spill value drip
    
    slay process() drip {
        damn value * 22
    }
    
    slay getId() drip {
        damn 22
    }
}

sus item22 Item22 = Item22{value: 110}
sus processed drip = item22.process()
vibez.spill("Interface 22:", processed)
