// Interface test 14
collab Processable14 {
    slay process() drip
    slay getId() drip
}

squad Item14 {
    spill value drip
    
    slay process() drip {
        damn value * 14
    }
    
    slay getId() drip {
        damn 14
    }
}

sus item14 Item14 = Item14{value: 70}
sus processed drip = item14.process()
vibez.spill("Interface 14:", processed)
