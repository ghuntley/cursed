// Interface test 7
collab Processable7 {
    slay process() drip
    slay getId() drip
}

squad Item7 {
    spill value drip
    
    slay process() drip {
        damn value * 7
    }
    
    slay getId() drip {
        damn 7
    }
}

sus item7 Item7 = Item7{value: 35}
sus processed drip = item7.process()
vibez.spill("Interface 7:", processed)
