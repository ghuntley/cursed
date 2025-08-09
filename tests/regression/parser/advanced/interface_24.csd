// Interface test 24
collab Processable24 {
    slay process() drip
    slay getId() drip
}

squad Item24 {
    spill value drip
    
    slay process() drip {
        damn value * 24
    }
    
    slay getId() drip {
        damn 24
    }
}

sus item24 Item24 = Item24{value: 120}
sus processed drip = item24.process()
vibez.spill("Interface 24:", processed)
